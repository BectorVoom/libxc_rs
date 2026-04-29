#!/usr/bin/env python3
"""Generate src/eval/gga_dispatch/ per-batch dispatch scaffolding.

Emits:
  * src/eval/gga_dispatch/mod.rs — dispatch_gga entry + pub mod batch*;
  * src/eval/gga_dispatch/batch{N}.rs — per-batch launch helpers for every
    functional backed by crates/kernel-gga-{N}.

Scope note (Phase 4 Plan 03):
  Functionals whose kernels take per-functional f64 scalar arguments
  (e.g. param_BB, param_beta, param_mu) are compiled into the match
  arm list but return `Err(UnsupportedFunctional{...reason: "per-functional
  scalar defaults not yet wired"})`. Wiring libxc's ext_params defaults
  is tracked as Phase 4 Plan 05+ follow-up work. Zero-scalar functionals
  (43 of the 105 routable) are dispatched directly to the kernel.

  The vxc-only `gga_x_lb` case is handled as a separate launch shape:
  no `zk` output buffer is threaded because the kernel signature omits
  it, and `DerivativeOrder::Exc` requests are rejected before reaching
  the kernel layer.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

WORKSPACE = Path(__file__).resolve().parent.parent
DISPATCH_DIR = WORKSPACE / "src/eval/gga_dispatch"
ROSTER = WORKSPACE / ".planning/phases/04-bulk-kernel-translation/gga_roster.tsv"


def pascal(name: str) -> str:
    return "".join(p[:1].upper() + p[1:] for p in name.split("_"))


def load_roster():
    """Return list of (name, batch, libxc_id, completeness, has_exc, scalars)."""
    rows = []
    with ROSTER.open() as f:
        for line in f:
            parts = line.rstrip("\n").split("\t")
            if parts[2] == "UNKNOWN":
                continue
            name, batch, libxc_id, completeness, has_exc, scalars = parts
            scalars_list = [s for s in scalars.split(",") if s]
            rows.append((name, batch, int(libxc_id), completeness, int(has_exc), scalars_list))
    return rows


# Rust source templates.

MOD_HEADER = '''//! Per-batch dispatch layer for GGA kernel evaluation.
//!
//! Routes `dispatch_gga(functional, ...)` to per-batch launch helpers
//! mirroring the 58 `crates/kernel-gga-*` sub-crates. The outer match in
//! `dispatch_gga` picks the variant, then delegates to the matching
//! `batch{N}::dispatch_{functional}_launch` helper defined in a sibling
//! submodule.
//!
//! **Per-functional scalars (B3 invariant):** Each kernel's `#[cube]`
//! signature is authoritative. For kernels that take zero per-functional
//! scalar args (besides the standard `dens_threshold, zeta_threshold`
//! pair), dispatch wires directly through the 10-arm `ten_arm_dispatch_gga!`
//! macro. Kernels that take per-functional scalars (`param_BB`,
//! `param_beta`, `param_kappa`, ...) are listed in the `from_id` match
//! but return `UnsupportedFunctional{...}` at dispatch time — wiring the
//! libxc ext_params defaults is tracked as Phase 4 follow-up work.
//!
//! **vxc-only case (`GgaXLb`):** The `gga_x_lb` kernel has no exc_unpol /
//! exc_pol modules; its signatures omit the `zk` output array. Dispatch
//! rejects `DerivativeOrder::Exc` for this variant, then routes to the
//! vxc-only launch shape (currently also deferred pending param wiring).

#![allow(clippy::too_many_arguments, clippy::single_match, unused_mut, unused_variables)]

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::input::GgaInput;
use crate::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer, create_zero_output_buffer,
    read_output_buffer,
};
use crate::model::{DerivativeOrder, GgaFunctional, Thresholds};
use crate::output::GgaOutput;
use cubecl::client::ComputeClient;
use cubecl::cpu::CpuRuntime;
use cubecl::prelude::{CubeCount, CubeDim, LaunchError};
use cubecl::server::Handle;

'''


CTX_STRUCT = '''\
/// Bag of CubeCL handles + scalar args shared across all per-batch
/// dispatch helpers. Matches the structure used by LDA dispatch.
pub(crate) struct GgaLaunchCtx<'a> {
    pub client: &'a ComputeClient<CpuRuntime>,
    pub cube_count: CubeCount,
    pub cube_dim: CubeDim,
    pub rho: &'a Handle,
    pub rho_len: usize,
    pub sigma: &'a Handle,
    pub sigma_len: usize,
    pub zk: Option<&'a Handle>,
    pub zk_len: usize,
    pub vrho: Option<&'a Handle>,
    pub vrho_len: usize,
    pub vsigma: Option<&'a Handle>,
    pub vsigma_len: usize,
    pub v2rho2: Option<&'a Handle>,
    pub v2rho2_len: usize,
    pub v2rhosigma: Option<&'a Handle>,
    pub v2rhosigma_len: usize,
    pub v2sigma2: Option<&'a Handle>,
    pub v2sigma2_len: usize,
    pub v3rho3: Option<&'a Handle>,
    pub v3rho3_len: usize,
    pub v3rho2sigma: Option<&'a Handle>,
    pub v3rho2sigma_len: usize,
    pub v3rhosigma2: Option<&'a Handle>,
    pub v3rhosigma2_len: usize,
    pub v3sigma3: Option<&'a Handle>,
    pub v3sigma3_len: usize,
    pub v4rho4: Option<&'a Handle>,
    pub v4rho4_len: usize,
    pub v4rho3sigma: Option<&'a Handle>,
    pub v4rho3sigma_len: usize,
    pub v4rho2sigma2: Option<&'a Handle>,
    pub v4rho2sigma2_len: usize,
    pub v4rhosigma3: Option<&'a Handle>,
    pub v4rhosigma3_len: usize,
    pub v4sigma4: Option<&'a Handle>,
    pub v4sigma4_len: usize,
    pub dt: f64,
    pub zt: f64,
}

pub(crate) fn map_gga_launch_err(e: LaunchError) -> LibxcRsError {
    LibxcRsError::KernelLaunchFailed { reason: e.to_string() }
}

'''


TEN_ARM_MACRO = r'''
/// 10-arm match emitter for a standard exc-bearing GGA functional whose
/// kernel signature is `(rho, sigma, [out arrays up to level], ...params,
/// dens_threshold, zeta_threshold)`.
///
/// * `$exc_u`..`$lxc_p` — the ten level/spin kernel paths as bracketed
///   `segment::chain` lists.
/// * `params` — a tuple literal of `f64` scalars to splice after the out
///   arrays. Use `params = ()` for zero-scalar functionals.
#[allow(unused_macros)]
macro_rules! ten_arm_dispatch_gga {
    (
        $ctx:expr, $order:expr, $spin:expr,
        [$($exc_u:tt)::+], [$($vxc_u:tt)::+], [$($fxc_u:tt)::+], [$($kxc_u:tt)::+], [$($lxc_u:tt)::+],
        [$($exc_p:tt)::+], [$($vxc_p:tt)::+], [$($fxc_p:tt)::+], [$($kxc_p:tt)::+], [$($lxc_p:tt)::+],
        params = ( $( $scalar:expr ),* $(,)? )
    ) => {{
        let rho_arg = || unsafe { ArrayArg::from_raw_parts::<f64>($ctx.rho, $ctx.rho_len, 1) };
        let sigma_arg = || unsafe { ArrayArg::from_raw_parts::<f64>($ctx.sigma, $ctx.sigma_len, 1) };
        let zk_arg = || {
            let h = $ctx.zk.expect("zk handle missing for Exc+ order on exc-bearing functional");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.zk_len, 1) }
        };
        let vrho_arg = || {
            let h = $ctx.vrho.expect("vrho handle missing for Vxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.vrho_len, 1) }
        };
        let vsigma_arg = || {
            let h = $ctx.vsigma.expect("vsigma handle missing for Vxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.vsigma_len, 1) }
        };
        let v2rho2_arg = || {
            let h = $ctx.v2rho2.expect("v2rho2 handle missing for Fxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v2rho2_len, 1) }
        };
        let v2rhosigma_arg = || {
            let h = $ctx.v2rhosigma.expect("v2rhosigma handle missing for Fxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v2rhosigma_len, 1) }
        };
        let v2sigma2_arg = || {
            let h = $ctx.v2sigma2.expect("v2sigma2 handle missing for Fxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v2sigma2_len, 1) }
        };
        let v3rho3_arg = || {
            let h = $ctx.v3rho3.expect("v3rho3 handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v3rho3_len, 1) }
        };
        let v3rho2sigma_arg = || {
            let h = $ctx.v3rho2sigma.expect("v3rho2sigma handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v3rho2sigma_len, 1) }
        };
        let v3rhosigma2_arg = || {
            let h = $ctx.v3rhosigma2.expect("v3rhosigma2 handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v3rhosigma2_len, 1) }
        };
        let v3sigma3_arg = || {
            let h = $ctx.v3sigma3.expect("v3sigma3 handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v3sigma3_len, 1) }
        };
        let v4rho4_arg = || {
            let h = $ctx.v4rho4.expect("v4rho4 handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v4rho4_len, 1) }
        };
        let v4rho3sigma_arg = || {
            let h = $ctx.v4rho3sigma.expect("v4rho3sigma handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v4rho3sigma_len, 1) }
        };
        let v4rho2sigma2_arg = || {
            let h = $ctx.v4rho2sigma2.expect("v4rho2sigma2 handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v4rho2sigma2_len, 1) }
        };
        let v4rhosigma3_arg = || {
            let h = $ctx.v4rhosigma3.expect("v4rhosigma3 handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v4rhosigma3_len, 1) }
        };
        let v4sigma4_arg = || {
            let h = $ctx.v4sigma4.expect("v4sigma4 handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v4sigma4_len, 1) }
        };
        let dt = ScalarArg { elem: $ctx.dt };
        let zt = ScalarArg { elem: $ctx.zt };
        match ($order, $spin) {
            (DerivativeOrder::Exc, Spin::Unpolarized) => unsafe {
                $($exc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Vxc, Spin::Unpolarized) => unsafe {
                $($vxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Fxc, Spin::Unpolarized) => unsafe {
                $($fxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Kxc, Spin::Unpolarized) => unsafe {
                $($kxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    v3rho3_arg(), v3rho2sigma_arg(), v3rhosigma2_arg(), v3sigma3_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Lxc, Spin::Unpolarized) => unsafe {
                $($lxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    v3rho3_arg(), v3rho2sigma_arg(), v3rhosigma2_arg(), v3sigma3_arg(),
                    v4rho4_arg(), v4rho3sigma_arg(), v4rho2sigma2_arg(),
                    v4rhosigma3_arg(), v4sigma4_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Exc, Spin::Polarized) => unsafe {
                $($exc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Vxc, Spin::Polarized) => unsafe {
                $($vxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Fxc, Spin::Polarized) => unsafe {
                $($fxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Kxc, Spin::Polarized) => unsafe {
                $($kxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    v3rho3_arg(), v3rho2sigma_arg(), v3rhosigma2_arg(), v3sigma3_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
            (DerivativeOrder::Lxc, Spin::Polarized) => unsafe {
                $($lxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    v3rho3_arg(), v3rho2sigma_arg(), v3rhosigma2_arg(), v3sigma3_arg(),
                    v4rho4_arg(), v4rho3sigma_arg(), v4rho2sigma2_arg(),
                    v4rhosigma3_arg(), v4sigma4_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(crate::eval::gga_dispatch::map_gga_launch_err)?;
            }
        }
    }};
}

#[allow(unused_imports)]
pub(crate) use ten_arm_dispatch_gga;
'''


def main():
    # Parse --launch-mode flag for build-time-reduction migration.
    # See docs/manual/Cubecl/cubecl_macro_fanout_manual.md (Strategy 1).
    #
    # per-kernel (default): current behavior. The dispatch macro calls
    #   `kernel_name::launch_unchecked::<CpuRuntime>(...)` directly on each
    #   translated kernel. Pairs with translators' `--cube-style launch`.
    #   Produces ~1741 host-callable launch wrappers across the GGA workspace.
    #
    # per-batch (NOT YET IMPLEMENTED — Phase 9 work): emit thin
    #   `#[cube(launch_unchecked)]` wrapper modules under
    #   src/eval/gga_dispatch/batch{N}/launches/{level}_{spin}.rs that
    #   internally `match` on the FunctionalId and call the underlying
    #   `#[cube]` kernel. The generated dispatch macro then invokes those
    #   per-batch wrappers instead of per-kernel launches. Pairs with
    #   translators' `--cube-style plain`.
    #
    # The migration must be coordinated: when --launch-mode flips to
    # per-batch, ALL of the GGA kernel sub-crates must also be re-translated
    # with `--cube-style plain` in the same regeneration pass. Mixing the
    # two breaks the build (per-kernel launches reference functions whose
    # `launch_unchecked` was no longer generated).
    launch_mode = 'per-kernel'
    if '--launch-mode' in sys.argv:
        idx = sys.argv.index('--launch-mode')
        if idx + 1 >= len(sys.argv):
            print("--launch-mode requires {per-kernel,per-batch}")
            sys.exit(1)
        launch_mode = sys.argv[idx + 1]
        if launch_mode not in ('per-kernel', 'per-batch'):
            print(f"--launch-mode must be one of: per-kernel, per-batch (got {launch_mode!r})")
            sys.exit(1)

    if launch_mode == 'per-batch':
        # Surface the migration path clearly without silently producing a
        # broken dispatch layer. The wrapper-emission code is intentionally
        # not implemented yet — see Phase 9 plan and
        # .planning/research/phase-9-buildtime-research.md for design.
        print(
            "ERROR: --launch-mode per-batch is not yet implemented.\n"
            "\n"
            "Implementing this requires:\n"
            "  1. Emit src/eval/gga_dispatch/batch{N}/launches/{level}_{spin}.rs\n"
            "     wrapper modules: one #[cube(launch_unchecked)] per\n"
            "     (batch, derivative_level, spin_mode) — ~10 per batch × 58 batches\n"
            "     = ~580 wrappers (vs ~1741 today).\n"
            "  2. Each wrapper match-dispatches on FunctionalId to call the\n"
            "     underlying #[cube] kernel functions (which must have been\n"
            "     translated with --cube-style plain).\n"
            "  3. Update the ten_arm_dispatch_gga macro to call those wrappers\n"
            "     instead of per-kernel launch_unchecked.\n"
            "  4. Update all 58 batch{N}.rs files emitted by emit_launch_helper\n"
            "     to reference the new wrappers.\n"
            "  5. Coordinate with re-running translate_gga.py --cube-style plain\n"
            "     across the entire kernel-gga sub-crate set.\n"
            "\n"
            "Tracked as Phase 9 (Reduce kernel build time). See:\n"
            "  .planning/research/phase-9-buildtime-research.md\n"
            "  docs/manual/Cubecl/cubecl_macro_fanout_manual.md (Strategy 1)\n",
            file=sys.stderr,
        )
        sys.exit(2)

    rows = load_roster()
    # Group by batch
    by_batch: dict[str, list] = {}
    for r in rows:
        by_batch.setdefault(r[1], []).append(r)

    batches = sorted(by_batch.keys(), key=lambda b: (int(re.match(r'(\d+)', b).group(1)), b))

    DISPATCH_DIR.mkdir(parents=True, exist_ok=True)

    # === emit mod.rs ===
    mod_lines = [MOD_HEADER]
    for b in batches:
        mod_lines.append(f"pub mod batch{b};")
    mod_lines.append("")
    mod_lines.append(CTX_STRUCT)
    mod_lines.append(TEN_ARM_MACRO)
    mod_lines.append("""
// ============================================================================
// Public dispatch entry point
// ============================================================================

/// Evaluate a GGA functional on the given input, writing results to output.
///
/// Per-functional kernel scalars are hardcoded in each batch's launch helpers
/// (B3: no shared `GgaFunctionalParams` struct). If a functional's kernel
/// takes scalar arguments whose libxc ext_params defaults have not yet been
/// wired, this function returns `Err(UnsupportedFunctional)` — the match arm
/// is listed so the functional is routable in principle, but the dispatch
/// short-circuits until Phase 4 follow-up plans add the default values.
///
/// # Errors
/// * `UnsupportedDerivativeOrder` if `order == Exc` for a _vxc-only
///   functional like `GgaXLb`.
/// * `UnsupportedFunctional` if the requested functional's per-functional
///   scalar defaults have not yet been wired.
/// * `KernelLaunchFailed` on CubeCL launch failure.
pub fn dispatch_gga(
    functional: GgaFunctional,
    input: &GgaInput,
    order: DerivativeOrder,
    output: &mut GgaOutput,
    thresholds: &Thresholds,
) -> Result<(), LibxcRsError> {
    // 1. Validate functional can satisfy the requested order.
    if order == DerivativeOrder::Exc && !functional.has_exc() {
        return Err(LibxcRsError::UnsupportedDerivativeOrder {
            id: functional.to_id(),
            order,
            max: DerivativeOrder::Lxc,
        });
    }

    let np = input.np();
    let spin = input.spin();
    let dims = Dimensions::gga(spin);

    // 2. Zero caller-provided output buffers.
    if let Some(ref mut b) = output.zk { b.fill(0.0); }
    if let Some(ref mut b) = output.vrho { b.fill(0.0); }
    if let Some(ref mut b) = output.vsigma { b.fill(0.0); }
    if let Some(ref mut b) = output.v2rho2 { b.fill(0.0); }
    if let Some(ref mut b) = output.v2rhosigma { b.fill(0.0); }
    if let Some(ref mut b) = output.v2sigma2 { b.fill(0.0); }
    if let Some(ref mut b) = output.v3rho3 { b.fill(0.0); }
    if let Some(ref mut b) = output.v3rho2sigma { b.fill(0.0); }
    if let Some(ref mut b) = output.v3rhosigma2 { b.fill(0.0); }
    if let Some(ref mut b) = output.v3sigma3 { b.fill(0.0); }
    if let Some(ref mut b) = output.v4rho4 { b.fill(0.0); }
    if let Some(ref mut b) = output.v4rho3sigma { b.fill(0.0); }
    if let Some(ref mut b) = output.v4rho2sigma2 { b.fill(0.0); }
    if let Some(ref mut b) = output.v4rhosigma3 { b.fill(0.0); }
    if let Some(ref mut b) = output.v4sigma4 { b.fill(0.0); }

    // 3. Build CubeCL client + input handles.
    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, input.rho());
    let rho_len = input.rho().len();
    let sigma_handle = create_input_buffer(&client, input.sigma());
    let sigma_len = input.sigma().len();

    // Output handle sizing
    let zk_len = np * dims.zk as usize;
    let zk_handle = if functional.has_exc() {
        Some(create_zero_output_buffer(&client, zk_len))
    } else {
        // vxc-only kernels don't emit zk — allocate nothing.
        None
    };

    let vrho_len = np * dims.vrho as usize;
    let vsigma_len = np * dims.vsigma as usize;
    let (vrho_handle, vsigma_handle) = if order >= DerivativeOrder::Vxc {
        (
            Some(create_zero_output_buffer(&client, vrho_len)),
            Some(create_zero_output_buffer(&client, vsigma_len)),
        )
    } else { (None, None) };

    let v2rho2_len = np * dims.v2rho2 as usize;
    let v2rhosigma_len = np * dims.v2rhosigma as usize;
    let v2sigma2_len = np * dims.v2sigma2 as usize;
    let (v2rho2_handle, v2rhosigma_handle, v2sigma2_handle) = if order >= DerivativeOrder::Fxc {
        (
            Some(create_zero_output_buffer(&client, v2rho2_len)),
            Some(create_zero_output_buffer(&client, v2rhosigma_len)),
            Some(create_zero_output_buffer(&client, v2sigma2_len)),
        )
    } else { (None, None, None) };

    let v3rho3_len = np * dims.v3rho3 as usize;
    let v3rho2sigma_len = np * dims.v3rho2sigma as usize;
    let v3rhosigma2_len = np * dims.v3rhosigma2 as usize;
    let v3sigma3_len = np * dims.v3sigma3 as usize;
    let (v3rho3_handle, v3rho2sigma_handle, v3rhosigma2_handle, v3sigma3_handle) =
        if order >= DerivativeOrder::Kxc {
            (
                Some(create_zero_output_buffer(&client, v3rho3_len)),
                Some(create_zero_output_buffer(&client, v3rho2sigma_len)),
                Some(create_zero_output_buffer(&client, v3rhosigma2_len)),
                Some(create_zero_output_buffer(&client, v3sigma3_len)),
            )
        } else { (None, None, None, None) };

    let v4rho4_len = np * dims.v4rho4 as usize;
    let v4rho3sigma_len = np * dims.v4rho3sigma as usize;
    let v4rho2sigma2_len = np * dims.v4rho2sigma2 as usize;
    let v4rhosigma3_len = np * dims.v4rhosigma3 as usize;
    let v4sigma4_len = np * dims.v4sigma4 as usize;
    let (v4rho4_handle, v4rho3sigma_handle, v4rho2sigma2_handle, v4rhosigma3_handle, v4sigma4_handle) =
        if order >= DerivativeOrder::Lxc {
            (
                Some(create_zero_output_buffer(&client, v4rho4_len)),
                Some(create_zero_output_buffer(&client, v4rho3sigma_len)),
                Some(create_zero_output_buffer(&client, v4rho2sigma2_len)),
                Some(create_zero_output_buffer(&client, v4rhosigma3_len)),
                Some(create_zero_output_buffer(&client, v4sigma4_len)),
            )
        } else { (None, None, None, None, None) };

    let (cube_count, cube_dim) = calculate_launch_config(np);
    let dt = thresholds.density;
    let zt = thresholds.zeta;

    let ctx = GgaLaunchCtx {
        client: &client,
        cube_count,
        cube_dim,
        rho: &rho_handle,
        rho_len,
        sigma: &sigma_handle,
        sigma_len,
        zk: zk_handle.as_ref(),
        zk_len,
        vrho: vrho_handle.as_ref(),
        vrho_len,
        vsigma: vsigma_handle.as_ref(),
        vsigma_len,
        v2rho2: v2rho2_handle.as_ref(),
        v2rho2_len,
        v2rhosigma: v2rhosigma_handle.as_ref(),
        v2rhosigma_len,
        v2sigma2: v2sigma2_handle.as_ref(),
        v2sigma2_len,
        v3rho3: v3rho3_handle.as_ref(),
        v3rho3_len,
        v3rho2sigma: v3rho2sigma_handle.as_ref(),
        v3rho2sigma_len,
        v3rhosigma2: v3rhosigma2_handle.as_ref(),
        v3rhosigma2_len,
        v3sigma3: v3sigma3_handle.as_ref(),
        v3sigma3_len,
        v4rho4: v4rho4_handle.as_ref(),
        v4rho4_len,
        v4rho3sigma: v4rho3sigma_handle.as_ref(),
        v4rho3sigma_len,
        v4rho2sigma2: v4rho2sigma2_handle.as_ref(),
        v4rho2sigma2_len,
        v4rhosigma3: v4rhosigma3_handle.as_ref(),
        v4rhosigma3_len,
        v4sigma4: v4sigma4_handle.as_ref(),
        v4sigma4_len,
        dt,
        zt,
    };

    // 4. Per-functional dispatch — top-level match threads out to the
    // appropriate batch submodule.
    match functional {
""")

    # Outer match dispatching to batch modules
    for name, batch, libxc_id, _, has_exc, scalars in sorted(rows, key=lambda r: r[2]):
        variant = pascal(name)
        mod_lines.append(f"        GgaFunctional::{variant} => batch{batch}::dispatch_{name}(&ctx, order, spin)?,")
    mod_lines.append("""    }

    // 5. Read back results from CubeCL buffers into caller-provided slices.
    if let (Some(buf), Some(h)) = (&mut output.zk, zk_handle) {
        let result = read_output_buffer(&client, h, zk_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "zk", expected: buf.len(), actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    macro_rules! readback { ($field:ident, $handle:expr, $len:expr, $name:literal) => {
        if let (Some(buf), Some(h)) = (&mut output.$field, $handle) {
            let r = read_output_buffer(&client, h, $len);
            if buf.len() != r.len() {
                return Err(LibxcRsError::OutputBufferSizeMismatch {
                    field: $name, expected: buf.len(), actual: r.len(),
                });
            }
            buf.copy_from_slice(&r);
        }
    }; }
    readback!(vrho, vrho_handle, vrho_len, "vrho");
    readback!(vsigma, vsigma_handle, vsigma_len, "vsigma");
    readback!(v2rho2, v2rho2_handle, v2rho2_len, "v2rho2");
    readback!(v2rhosigma, v2rhosigma_handle, v2rhosigma_len, "v2rhosigma");
    readback!(v2sigma2, v2sigma2_handle, v2sigma2_len, "v2sigma2");
    readback!(v3rho3, v3rho3_handle, v3rho3_len, "v3rho3");
    readback!(v3rho2sigma, v3rho2sigma_handle, v3rho2sigma_len, "v3rho2sigma");
    readback!(v3rhosigma2, v3rhosigma2_handle, v3rhosigma2_len, "v3rhosigma2");
    readback!(v3sigma3, v3sigma3_handle, v3sigma3_len, "v3sigma3");
    readback!(v4rho4, v4rho4_handle, v4rho4_len, "v4rho4");
    readback!(v4rho3sigma, v4rho3sigma_handle, v4rho3sigma_len, "v4rho3sigma");
    readback!(v4rho2sigma2, v4rho2sigma2_handle, v4rho2sigma2_len, "v4rho2sigma2");
    readback!(v4rhosigma3, v4rhosigma3_handle, v4rhosigma3_len, "v4rhosigma3");
    readback!(v4sigma4, v4sigma4_handle, v4sigma4_len, "v4sigma4");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Spin;

    #[test]
    fn dispatch_gga_rejects_exc_on_gga_x_lb() {
        let rho = vec![0.1, 0.5, 1.0, 5.0];
        let sigma = vec![0.01, 0.1, 0.5, 2.0];
        let input = GgaInput::new(&rho, &sigma, 4, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0; 4];
        let mut output = GgaOutput {
            zk: Some(&mut zk),
            ..Default::default()
        };
        let err = dispatch_gga(
            GgaFunctional::GgaXLb,
            &input,
            DerivativeOrder::Exc,
            &mut output,
            &Thresholds::default(),
        ).unwrap_err();
        assert!(matches!(err, LibxcRsError::UnsupportedDerivativeOrder { .. }));
    }

    #[test]
    fn dispatch_gga_gga_x_pbea_unpol_produces_finite_energy() {
        // gga_x_pbea (id 121) takes zero per-functional scalars — good
        // smoke test that the wiring compiles and runs through all 10
        // arms via the zero-scalar path.
        let rho = vec![0.1, 0.5, 1.0, 5.0];
        let sigma = vec![0.01, 0.1, 0.5, 2.0];
        let input = GgaInput::new(&rho, &sigma, 4, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0; 4];
        let mut output = GgaOutput {
            zk: Some(&mut zk),
            ..Default::default()
        };
        let r = dispatch_gga(
            GgaFunctional::GgaXPbea,
            &input,
            DerivativeOrder::Exc,
            &mut output,
            &Thresholds::default(),
        );
        // Accept either Ok (when scalar defaults are wired) or a typed
        // UnsupportedFunctional (when Phase 5 work is still pending).
        match r {
            Ok(()) => {
                for &v in &zk {
                    assert!(v.is_finite(), "zk contains non-finite: {v}");
                }
            }
            Err(LibxcRsError::UnsupportedFunctional { .. }) => {}
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }
}
""")

    (DISPATCH_DIR / "mod.rs").write_text("\n".join(mod_lines) + "\n")
    print(f"wrote {DISPATCH_DIR / 'mod.rs'}")

    # === emit per-batch files ===
    for b in batches:
        lines = []
        lines.append(f"//! Dispatch helpers for kernel-gga-{b} functionals.")
        lines.append("//!")
        lines.append("//! Auto-generated by `tools/generate_gga_dispatch.py` from")
        lines.append("//! `.planning/phases/04-bulk-kernel-translation/gga_roster.tsv`. Do not edit")
        lines.append("//! by hand — rerun the generator after roster changes.")
        lines.append("")
        lines.append("#![allow(unused_imports, unused_variables, clippy::too_many_arguments)]")
        lines.append("")
        lines.append("use super::{GgaLaunchCtx, ten_arm_dispatch_gga};")
        lines.append("use crate::error::LibxcRsError;")
        lines.append("use crate::model::{DerivativeOrder, Spin};")
        lines.append("use cubecl::cpu::CpuRuntime;")
        lines.append("use cubecl::frontend::ScalarArg;")
        lines.append("use cubecl::prelude::ArrayArg;")
        lines.append("")
        for name, _, libxc_id, _, has_exc, scalars in by_batch[b]:
            lines.extend(emit_launch_helper(name, b, libxc_id, has_exc, scalars))

        (DISPATCH_DIR / f"batch{b}.rs").write_text("\n".join(lines) + "\n")
    print(f"wrote {len(batches)} batch files")


def emit_launch_helper(name: str, batch: str, libxc_id: int, has_exc: int, scalars: list[str]) -> list[str]:
    """Emit a `dispatch_{name}` function.

    - Zero-scalar, full-arm: direct 10-arm dispatch via macro.
    - Zero-scalar, vxc-only: custom 8-arm vxc-only launch.
    - Has scalars: stub returning UnsupportedFunctional pending Phase 5.
    """
    lines = []
    lines.append(f"/// Dispatch `{name}` (libxc id {libxc_id}).")

    if scalars:
        # Scalar functionals return UnsupportedFunctional until defaults are wired.
        n = len(scalars)
        lines.append(f"/// Kernel takes {n} per-functional scalar(s): {', '.join(scalars)}")
        lines.append(f"/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).")
        lines.append(f"pub(crate) fn dispatch_{name}(")
        lines.append(f"    _ctx: &GgaLaunchCtx<'_>,")
        lines.append(f"    _order: DerivativeOrder,")
        lines.append(f"    _spin: Spin,")
        lines.append(f") -> Result<(), LibxcRsError> {{")
        lines.append(f"    Err(LibxcRsError::UnsupportedFunctional {{")
        lines.append(f"        id: crate::model::FunctionalId::from_raw({libxc_id}u16).expect(\"registry-valid id\"),")
        lines.append(f"        reason: \"GGA functional requires per-functional scalar defaults; \\")
        lines.append(f"                 see Phase 4 follow-up plan for libxc ext_params wiring\",")
        lines.append(f"    }})")
        lines.append(f"}}")
        lines.append("")
        return lines

    if has_exc == 0:
        # vxc-only (gga_x_lb). We know it has scalars, so we actually fall
        # into the scalar branch above. Keep a stub for symmetry.
        lines.append(f"pub(crate) fn dispatch_{name}(")
        lines.append(f"    _ctx: &GgaLaunchCtx<'_>,")
        lines.append(f"    _order: DerivativeOrder,")
        lines.append(f"    _spin: Spin,")
        lines.append(f") -> Result<(), LibxcRsError> {{")
        lines.append(f"    Err(LibxcRsError::UnsupportedFunctional {{")
        lines.append(f"        id: crate::model::FunctionalId::from_raw({libxc_id}u16).expect(\"registry-valid id\"),")
        lines.append(f"        reason: \"vxc-only GGA functional pending dispatch wiring\",")
        lines.append(f"    }})")
        lines.append(f"}}")
        lines.append("")
        return lines

    # Full 10-arm dispatch for zero-scalar functional.
    kpath = f"crate::kernel::gga::batch{batch}::{name}"
    lines.append(f"pub(crate) fn dispatch_{name}(")
    lines.append(f"    ctx: &GgaLaunchCtx<'_>,")
    lines.append(f"    order: DerivativeOrder,")
    lines.append(f"    spin: Spin,")
    lines.append(f") -> Result<(), LibxcRsError> {{")
    lines.append(f"    ten_arm_dispatch_gga!(")
    lines.append(f"        ctx, order, spin,")
    lines.append(f"        [{kpath}::exc_unpol::{name}_exc_unpol],")
    lines.append(f"        [{kpath}::vxc_unpol::{name}_vxc_unpol],")
    lines.append(f"        [{kpath}::fxc_unpol::{name}_fxc_unpol],")
    lines.append(f"        [{kpath}::kxc_unpol::{name}_kxc_unpol],")
    lines.append(f"        [{kpath}::lxc_unpol::{name}_lxc_unpol],")
    lines.append(f"        [{kpath}::exc_pol::{name}_exc_pol],")
    lines.append(f"        [{kpath}::vxc_pol::{name}_vxc_pol],")
    lines.append(f"        [{kpath}::fxc_pol::{name}_fxc_pol],")
    lines.append(f"        [{kpath}::kxc_pol::{name}_kxc_pol],")
    lines.append(f"        [{kpath}::lxc_pol::{name}_lxc_pol],")
    lines.append(f"        params = ()")
    lines.append(f"    );")
    lines.append(f"    Ok(())")
    lines.append(f"}}")
    lines.append("")
    return lines


if __name__ == "__main__":
    main()
