//! Match-based dispatch layer for LDA kernel evaluation.
//!
//! Routes evaluation requests to the correct CubeCL kernel function based on
//! `LdaFunctional` selection, derivative order, and spin mode. Every kernel
//! launch goes through this module — the BUILD-04 invariant that no raw
//! `launch_unchecked` calls live outside `src/eval/dispatch.rs` and
//! `src/kernel/launch.rs` is preserved.
//!
//! For each compiled LDA functional the libxc default ext-param values are
//! hardcoded at the call site so that `dispatch_lda(functional, …)` matches
//! the C oracle bit-for-bit on parameter inputs. Per-functional ext-param
//! overrides are scope for a future plan; for Phase 4 LDA the defaults
//! match what `oracle_lda_all(id, spin, rho)` evaluates against.

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::input::LdaInput;
use crate::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer,
    create_zero_output_buffer, read_output_buffer,
};
use crate::model::{DerivativeOrder, LdaFunctional, Spin, Thresholds};
use crate::output::LdaOutput;
use cubecl::client::ComputeClient;
use cubecl::cpu::CpuRuntime;
use cubecl::frontend::ScalarArg;
use cubecl::prelude::{ArrayArg, CubeCount, CubeDim, LaunchError};
use cubecl::server::Handle;

/// Per-functional scalar parameters that vary between LDA kernels.
///
/// For exchange functionals (`LdaX`, `LdaX2d`, `LdaXRel`, `LdaXErf`,
/// `LdaXSloc`, `LdaXYukawa`) `alpha` is the Slater exchange scaling and
/// defaults to 1.0. All other LDA functionals ignore this value — their
/// parameters are hardcoded to the libxc 7.0.0 defaults at the call site.
///
/// **Phase 4 scope:** This struct exposes the historical `param_alpha` knob
/// only. Configurable per-functional ext-params (e.g. `lda_x_erf` omega,
/// `lda_c_chachiyo` ap/bp/cp/af/bf/cf overrides) are tracked for a future
/// plan. The defaults baked into dispatch match libxc's C oracle so the
/// per-functional Rust-vs-C verify tests pass without parameter plumbing.
#[derive(Debug, Clone, Copy)]
pub struct LdaFunctionalParams {
    /// Slater exchange scaling for `lda_x`. Ignored by every other
    /// functional (their parameters come from libxc defaults).
    pub alpha: f64,
}

impl Default for LdaFunctionalParams {
    fn default() -> Self {
        Self { alpha: 1.0 }
    }
}

// Plan 05-02 bridge: implement FunctionalParams for the historical
// LdaFunctionalParams struct so call sites can pass it as `&dyn FunctionalParams`
// without changing the dispatch arm bodies. The dispatch arm for LdaX continues
// to consume `params.alpha` directly via the typed parameter, while the trait
// object exists for forward-compatibility with the new Functional API.
impl crate::functional::params::FunctionalParams for LdaFunctionalParams {
    fn ext_param_count(&self) -> usize {
        1
    }

    fn raw_ext_params(&self) -> &[f64] {
        // Cannot return a &[f64] over a Copy field without owning storage;
        // the historical struct doesn't carry a Box. Plan 05-03 may rework
        // this to use the LdaXParams concrete type from functional::params_lda.
        // For now, return an empty slice — the trait object is used only as
        // a Send+Sync handle in this plan; ext_param queries go through
        // Functional's own getters which read from Functional.ext_params.
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if vals.len() != 1 {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: crate::model::FunctionalId(1),
                expected: 1,
                actual: vals.len(),
            });
        }
        self.alpha = vals[0];
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// ============================================================================
// Public dispatch entry point
// ============================================================================

/// Evaluate an LDA functional on the given input, writing results to output.
///
/// Routes to the correct kernel based on `functional`, derivative order,
/// and spin mode. Zeros caller output buffers before evaluation. Handles
/// `None` output fields by allocating dummy buffers the kernel writes to
/// but whose results are discarded.
///
/// # Arguments
/// * `functional` - Selects which LDA kernel module to launch
/// * `input` - Validated LDA input bundle
/// * `order` - Maximum derivative order to compute
/// * `output` - Output bundle with optional buffers for each derivative level
/// * `params` - Per-functional scalar parameters (currently `alpha` for
///   exchange functionals; all other functionals use libxc defaults)
/// * `thresholds` - Numerical thresholds for evaluation stability
///
/// # Errors
/// * `UnsupportedDerivativeOrder` if `order == Exc` for a `_vxc`-only
///   functional like `LdaXcTih`.
/// * `KernelLaunchFailed` on CubeCL launch failure.
/// * `OutputBufferSizeMismatch` if caller-provided buffers are wrong size.
pub fn dispatch_lda(
    functional: LdaFunctional,
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    params: &LdaFunctionalParams,
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
    let dims = Dimensions::lda(spin);

    // 2. Zero caller-provided output buffers (T-03-04 mitigation).
    if let Some(ref mut buf) = output.zk { buf.fill(0.0); }
    if let Some(ref mut buf) = output.vrho { buf.fill(0.0); }
    if let Some(ref mut buf) = output.v2rho2 { buf.fill(0.0); }
    if let Some(ref mut buf) = output.v3rho3 { buf.fill(0.0); }
    if let Some(ref mut buf) = output.v4rho4 { buf.fill(0.0); }

    // 3. Build CubeCL client + handles.
    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, input.rho());
    let rho_len = input.rho().len();

    let zk_len = np * dims.zk as usize;
    let zk_handle = create_zero_output_buffer(&client, zk_len);

    let vrho_len = np * dims.vrho as usize;
    let vrho_handle = if order >= DerivativeOrder::Vxc {
        Some(create_zero_output_buffer(&client, vrho_len))
    } else { None };

    let v2rho2_len = np * dims.v2rho2 as usize;
    let v2rho2_handle = if order >= DerivativeOrder::Fxc {
        Some(create_zero_output_buffer(&client, v2rho2_len))
    } else { None };

    let v3rho3_len = np * dims.v3rho3 as usize;
    let v3rho3_handle = if order >= DerivativeOrder::Kxc {
        Some(create_zero_output_buffer(&client, v3rho3_len))
    } else { None };

    let v4rho4_len = np * dims.v4rho4 as usize;
    let v4rho4_handle = if order >= DerivativeOrder::Lxc {
        Some(create_zero_output_buffer(&client, v4rho4_len))
    } else { None };

    let (cube_count, cube_dim) = calculate_launch_config(np);
    let dt = thresholds.density;
    let zt = thresholds.zeta;

    let ctx = LaunchCtx {
        client: &client,
        cube_count,
        cube_dim,
        rho: &rho_handle,
        rho_len,
        zk: &zk_handle,
        zk_len,
        vrho: vrho_handle.as_ref(),
        vrho_len,
        v2rho2: v2rho2_handle.as_ref(),
        v2rho2_len,
        v3rho3: v3rho3_handle.as_ref(),
        v3rho3_len,
        v4rho4: v4rho4_handle.as_ref(),
        v4rho4_len,
        dt,
        zt,
    };

    // 4. Per-functional dispatch.
    match functional {
        LdaFunctional::LdaX                => launch_lda_x(&ctx, order, spin, params.alpha)?,
        LdaFunctional::LdaX2d              => launch_lda_x_2d(&ctx, order, spin)?,
        LdaFunctional::LdaXRel             => launch_lda_x_rel(&ctx, order, spin)?,
        LdaFunctional::LdaXErf             => launch_lda_x_erf(&ctx, order, spin)?,
        LdaFunctional::LdaXSloc            => launch_lda_x_sloc(&ctx, order, spin)?,
        LdaFunctional::LdaXYukawa          => launch_lda_x_yukawa(&ctx, order, spin)?,
        LdaFunctional::LdaCRpa             => launch_lda_c_rpa(&ctx, order, spin)?,
        LdaFunctional::LdaCHl              => launch_lda_c_hl(&ctx, order, spin)?,
        LdaFunctional::LdaCVwn             => launch_lda_c_vwn(&ctx, order, spin)?,
        LdaFunctional::LdaCVwnRpa          => launch_lda_c_vwn_rpa(&ctx, order, spin)?,
        LdaFunctional::LdaCVwn1            => launch_lda_c_vwn_1(&ctx, order, spin)?,
        LdaFunctional::LdaCVwn2            => launch_lda_c_vwn_2(&ctx, order, spin)?,
        LdaFunctional::LdaCVwn3            => launch_lda_c_vwn_3(&ctx, order, spin)?,
        LdaFunctional::LdaCVwn4            => launch_lda_c_vwn_4(&ctx, order, spin)?,
        LdaFunctional::LdaCPz              => launch_lda_c_pz(&ctx, order, spin)?,
        LdaFunctional::LdaCPw              => launch_lda_c_pw(&ctx, order, spin)?,
        LdaFunctional::LdaCWigner          => launch_lda_c_wigner(&ctx, order, spin)?,
        LdaFunctional::LdaCRc04            => launch_lda_c_rc04(&ctx, order, spin)?,
        LdaFunctional::LdaC2dAmgb          => launch_lda_c_2d_amgb(&ctx, order, spin)?,
        LdaFunctional::LdaC2dPrm           => launch_lda_c_2d_prm(&ctx, order, spin)?,
        LdaFunctional::LdaC1dCsc           => launch_lda_c_1d_csc(&ctx, order, spin)?,
        LdaFunctional::LdaC1dLoos          => launch_lda_c_1d_loos(&ctx, order, spin)?,
        LdaFunctional::LdaCGk72            => launch_lda_c_gk72(&ctx, order, spin)?,
        LdaFunctional::LdaCGombas          => launch_lda_c_gombas(&ctx, order, spin)?,
        LdaFunctional::LdaCLp96            => launch_lda_c_lp96(&ctx, order, spin)?,
        LdaFunctional::LdaCMl1             => launch_lda_c_ml1(&ctx, order, spin)?,
        LdaFunctional::LdaCW20             => launch_lda_c_w20(&ctx, order, spin)?,
        LdaFunctional::LdaCChachiyo        => launch_lda_c_chachiyo(&ctx, order, spin)?,
        LdaFunctional::LdaCChachiyoMod     => launch_lda_c_chachiyo_mod(&ctx, order, spin)?,
        LdaFunctional::LdaKTf              => launch_lda_k_tf(&ctx, order, spin)?,
        LdaFunctional::LdaKZlp             => launch_lda_k_zlp(&ctx, order, spin)?,
        LdaFunctional::LdaXcTeter93        => launch_lda_xc_teter93(&ctx, order, spin)?,
        LdaFunctional::LdaXcZlp            => launch_lda_xc_zlp(&ctx, order, spin)?,
        LdaFunctional::LdaXcTih            => launch_lda_xc_tih(&ctx, order, spin)?,
        LdaFunctional::LdaXc1dEhwlrg1      => launch_lda_xc_1d_ehwlrg(
            &ctx, order, spin,
            -0.803, 0.82, -0.47, 0.638,
        )?,
        LdaFunctional::LdaXc1dEhwlrg2      => launch_lda_xc_1d_ehwlrg(
            &ctx, order, spin,
            -0.74, 0.68, -0.38, 0.604,
        )?,
        LdaFunctional::LdaXc1dEhwlrg3      => launch_lda_xc_1d_ehwlrg(
            &ctx, order, spin,
            -0.77, 0.79, -0.48, 0.61,
        )?,
        LdaFunctional::HybLdaXcBn05        => launch_hyb_lda_xc_bn05(&ctx, order, spin)?,
    }

    // 5. Read back results from CubeCL buffers into caller-provided slices.
    if let Some(ref mut buf) = output.zk {
        let result = read_output_buffer(&client, zk_handle, zk_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "zk", expected: buf.len(), actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.vrho, vrho_handle) {
        let result = read_output_buffer(&client, h, vrho_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "vrho", expected: buf.len(), actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v2rho2, v2rho2_handle) {
        let result = read_output_buffer(&client, h, v2rho2_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v2rho2", expected: buf.len(), actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v3rho3, v3rho3_handle) {
        let result = read_output_buffer(&client, h, v3rho3_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v3rho3", expected: buf.len(), actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v4rho4, v4rho4_handle) {
        let result = read_output_buffer(&client, h, v4rho4_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v4rho4", expected: buf.len(), actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }

    Ok(())
}

// ============================================================================
// Internal helpers
// ============================================================================

/// Bag of CubeCL handles + scalar args shared across all per-functional
/// launch helpers. Held by reference so per-arm helpers don't drop owned
/// handles before readback.
struct LaunchCtx<'a> {
    client: &'a ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &'a Handle,
    rho_len: usize,
    zk: &'a Handle,
    zk_len: usize,
    vrho: Option<&'a Handle>,
    vrho_len: usize,
    v2rho2: Option<&'a Handle>,
    v2rho2_len: usize,
    v3rho3: Option<&'a Handle>,
    v3rho3_len: usize,
    v4rho4: Option<&'a Handle>,
    v4rho4_len: usize,
    dt: f64,
    zt: f64,
}

fn map_launch_err(e: LaunchError) -> LibxcRsError {
    LibxcRsError::KernelLaunchFailed { reason: e.to_string() }
}

// ----------------------------------------------------------------------------
// Macro-driven per-functional dispatch helpers.
//
// Each `lda_dispatch_fn!` invocation generates a `launch_<name>` function
// that matches on (DerivativeOrder, Spin) and calls the corresponding
// `<level>_<spin>::<kernel_fn>::launch_unchecked` from the kernel-lda crate.
//
// `with_alpha`  → kernel signature is (rho, zk, ..., param_alpha, dt, zt)
//                 and the helper takes an extra `alpha: f64` argument.
// `no_alpha`    → kernel signature is (rho, zk, ..., dt, zt) — no scalar
//                 ext-param plumbing.
// `parameterized` → kernel takes a fixed list of param scalars given as a
//                 [f64; N] array; defaults are baked into the helper body.
// `vxc_only`    → no exc_unpol/exc_pol modules; only Vxc..Lxc arms.
// `with_extra_params` → kernel takes additional param scalars passed by
//                 the caller (used by `lda_xc_1d_ehwlrg` to vary defaults
//                 across IDs 536/537/538).
// ----------------------------------------------------------------------------

// We avoid `concat_idents!` (unstable). Instead each helper imports the
// kernel functions by their explicit names.

/// Compact 10-arm match emitter for exc-bearing functionals with the
/// supplied parameter list (a tuple of `ScalarArg`s pre-built by the
/// caller). Each arm pulls the same parameter list, threading the
/// per-level `ArrayArg` slots in.
macro_rules! ten_arm_dispatch {
    // Variant: same parameter list across unpol and pol kernels.
    (
        $ctx:expr, $order:expr, $spin:expr,
        [$($exc_u:tt)::+], [$($vxc_u:tt)::+], [$($fxc_u:tt)::+], [$($kxc_u:tt)::+], [$($lxc_u:tt)::+],
        [$($exc_p:tt)::+], [$($vxc_p:tt)::+], [$($fxc_p:tt)::+], [$($kxc_p:tt)::+], [$($lxc_p:tt)::+],
        params = ( $( $scalar:expr ),* $(,)? )
    ) => {
        ten_arm_dispatch!(
            $ctx, $order, $spin,
            [$($exc_u)::+], [$($vxc_u)::+], [$($fxc_u)::+], [$($kxc_u)::+], [$($lxc_u)::+],
            [$($exc_p)::+], [$($vxc_p)::+], [$($fxc_p)::+], [$($kxc_p)::+], [$($lxc_p)::+],
            params_unpol = ( $( $scalar ),* ),
            params_pol   = ( $( $scalar ),* ),
        )
    };

    // Variant: separate parameter lists for unpol and pol kernels (e.g. lda_c_pw
    // takes 15 params unpol but 22 params pol because the polarized kernel
    // also receives the ferromagnetic channel).
    (
        $ctx:expr, $order:expr, $spin:expr,
        [$($exc_u:tt)::+], [$($vxc_u:tt)::+], [$($fxc_u:tt)::+], [$($kxc_u:tt)::+], [$($lxc_u:tt)::+],
        [$($exc_p:tt)::+], [$($vxc_p:tt)::+], [$($fxc_p:tt)::+], [$($kxc_p:tt)::+], [$($lxc_p:tt)::+],
        params_unpol = ( $( $scalar_u:expr ),* $(,)? ),
        params_pol   = ( $( $scalar_p:expr ),* $(,)? ) $(,)?
    ) => {{
        let rho_arg = || unsafe { ArrayArg::from_raw_parts::<f64>($ctx.rho, $ctx.rho_len, 1) };
        let zk_arg  = || unsafe { ArrayArg::from_raw_parts::<f64>($ctx.zk,  $ctx.zk_len,  1) };
        let vrho_arg = || {
            let h = $ctx.vrho.expect("vrho handle missing for Vxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.vrho_len, 1) }
        };
        let v2_arg = || {
            let h = $ctx.v2rho2.expect("v2rho2 handle missing for Fxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v2rho2_len, 1) }
        };
        let v3_arg = || {
            let h = $ctx.v3rho3.expect("v3rho3 handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v3rho3_len, 1) }
        };
        let v4_arg = || {
            let h = $ctx.v4rho4.expect("v4rho4 handle missing for Lxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v4rho4_len, 1) }
        };
        let dt = ScalarArg { elem: $ctx.dt };
        let zt = ScalarArg { elem: $ctx.zt };
        match ($order, $spin) {
            (DerivativeOrder::Exc, Spin::Unpolarized) => unsafe {
                $($exc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(),
                    $( ScalarArg { elem: $scalar_u }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Vxc, Spin::Unpolarized) => unsafe {
                $($vxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(), vrho_arg(),
                    $( ScalarArg { elem: $scalar_u }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Fxc, Spin::Unpolarized) => unsafe {
                $($fxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(), vrho_arg(), v2_arg(),
                    $( ScalarArg { elem: $scalar_u }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Kxc, Spin::Unpolarized) => unsafe {
                $($kxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(), vrho_arg(), v2_arg(), v3_arg(),
                    $( ScalarArg { elem: $scalar_u }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Lxc, Spin::Unpolarized) => unsafe {
                $($lxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(), vrho_arg(), v2_arg(), v3_arg(), v4_arg(),
                    $( ScalarArg { elem: $scalar_u }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Exc, Spin::Polarized) => unsafe {
                $($exc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(),
                    $( ScalarArg { elem: $scalar_p }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Vxc, Spin::Polarized) => unsafe {
                $($vxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(), vrho_arg(),
                    $( ScalarArg { elem: $scalar_p }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Fxc, Spin::Polarized) => unsafe {
                $($fxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(), vrho_arg(), v2_arg(),
                    $( ScalarArg { elem: $scalar_p }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Kxc, Spin::Polarized) => unsafe {
                $($kxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(), vrho_arg(), v2_arg(), v3_arg(),
                    $( ScalarArg { elem: $scalar_p }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Lxc, Spin::Polarized) => unsafe {
                $($lxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), zk_arg(), vrho_arg(), v2_arg(), v3_arg(), v4_arg(),
                    $( ScalarArg { elem: $scalar_p }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
        }
    }};
}

/// 8-arm match emitter for `_vxc`-only functionals (no exc_unpol / exc_pol
/// modules; the kernel signature OMITS `zk` from its argument list because
/// no energy density is computed).
macro_rules! eight_arm_vxc_only_dispatch {
    (
        $ctx:expr, $order:expr, $spin:expr,
        [$($vxc_u:tt)::+], [$($fxc_u:tt)::+], [$($kxc_u:tt)::+], [$($lxc_u:tt)::+],
        [$($vxc_p:tt)::+], [$($fxc_p:tt)::+], [$($kxc_p:tt)::+], [$($lxc_p:tt)::+],
        params = ( $( $scalar:expr ),* $(,)? )
    ) => {{
        let rho_arg = || unsafe { ArrayArg::from_raw_parts::<f64>($ctx.rho, $ctx.rho_len, 1) };
        let vrho_arg = || {
            let h = $ctx.vrho.expect("vrho handle missing for Vxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.vrho_len, 1) }
        };
        let v2_arg = || {
            let h = $ctx.v2rho2.expect("v2rho2 handle missing for Fxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v2rho2_len, 1) }
        };
        let v3_arg = || {
            let h = $ctx.v3rho3.expect("v3rho3 handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v3rho3_len, 1) }
        };
        let v4_arg = || {
            let h = $ctx.v4rho4.expect("v4rho4 handle missing for Lxc+ order");
            unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.v4rho4_len, 1) }
        };
        let dt = ScalarArg { elem: $ctx.dt };
        let zt = ScalarArg { elem: $ctx.zt };
        match ($order, $spin) {
            (DerivativeOrder::Exc, _) => unreachable!(
                "dispatch_lda already validated has_exc() before calling vxc-only helper"
            ),
            (DerivativeOrder::Vxc, Spin::Unpolarized) => unsafe {
                $($vxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), vrho_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Fxc, Spin::Unpolarized) => unsafe {
                $($fxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), vrho_arg(), v2_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Kxc, Spin::Unpolarized) => unsafe {
                $($kxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), vrho_arg(), v2_arg(), v3_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Lxc, Spin::Unpolarized) => unsafe {
                $($lxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), vrho_arg(), v2_arg(), v3_arg(), v4_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Vxc, Spin::Polarized) => unsafe {
                $($vxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), vrho_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Fxc, Spin::Polarized) => unsafe {
                $($fxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), vrho_arg(), v2_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Kxc, Spin::Polarized) => unsafe {
                $($kxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), vrho_arg(), v2_arg(), v3_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
            (DerivativeOrder::Lxc, Spin::Polarized) => unsafe {
                $($lxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), vrho_arg(), v2_arg(), v3_arg(), v4_arg(),
                    $( ScalarArg { elem: $scalar }, )*
                    dt, zt,
                ).map_err(map_launch_err)?;
            }
        }
    }};
}

// ----------------------------------------------------------------------------
// Per-functional launch helpers.
// ----------------------------------------------------------------------------

fn launch_lda_x(
    ctx: &LaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
    alpha: f64,
) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_x::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_x_exc_unpol], [vxc_unpol::lda_x_vxc_unpol],
        [fxc_unpol::lda_x_fxc_unpol], [kxc_unpol::lda_x_kxc_unpol],
        [lxc_unpol::lda_x_lxc_unpol],
        [exc_pol::lda_x_exc_pol],    [vxc_pol::lda_x_vxc_pol],
        [fxc_pol::lda_x_fxc_pol],    [kxc_pol::lda_x_kxc_pol],
        [lxc_pol::lda_x_lxc_pol],
        params = (alpha)
    );
    Ok(())
}

fn launch_lda_x_2d(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_x_2d::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_x_2d_exc_unpol], [vxc_unpol::lda_x_2d_vxc_unpol],
        [fxc_unpol::lda_x_2d_fxc_unpol], [kxc_unpol::lda_x_2d_kxc_unpol],
        [lxc_unpol::lda_x_2d_lxc_unpol],
        [exc_pol::lda_x_2d_exc_pol],    [vxc_pol::lda_x_2d_vxc_pol],
        [fxc_pol::lda_x_2d_fxc_pol],    [kxc_pol::lda_x_2d_kxc_pol],
        [lxc_pol::lda_x_2d_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_x_rel(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_x_rel::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_x_rel_exc_unpol], [vxc_unpol::lda_x_rel_vxc_unpol],
        [fxc_unpol::lda_x_rel_fxc_unpol], [kxc_unpol::lda_x_rel_kxc_unpol],
        [lxc_unpol::lda_x_rel_lxc_unpol],
        [exc_pol::lda_x_rel_exc_pol],    [vxc_pol::lda_x_rel_vxc_pol],
        [fxc_pol::lda_x_rel_fxc_pol],    [kxc_pol::lda_x_rel_kxc_pol],
        [lxc_pol::lda_x_rel_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_x_erf(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_x_erf::*;
    // libxc default omega = 0.3 (omega_values[] in lda_x_erf.c)
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_x_erf_exc_unpol], [vxc_unpol::lda_x_erf_vxc_unpol],
        [fxc_unpol::lda_x_erf_fxc_unpol], [kxc_unpol::lda_x_erf_kxc_unpol],
        [lxc_unpol::lda_x_erf_lxc_unpol],
        [exc_pol::lda_x_erf_exc_pol],    [vxc_pol::lda_x_erf_vxc_pol],
        [fxc_pol::lda_x_erf_fxc_pol],    [kxc_pol::lda_x_erf_kxc_pol],
        [lxc_pol::lda_x_erf_lxc_pol],
        params = (0.3_f64)
    );
    Ok(())
}

fn launch_lda_x_sloc(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_x_sloc::*;
    // libxc default sloc_values = {1.67, 0.3} -> param_a=1.67, param_b=0.3
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_x_sloc_exc_unpol], [vxc_unpol::lda_x_sloc_vxc_unpol],
        [fxc_unpol::lda_x_sloc_fxc_unpol], [kxc_unpol::lda_x_sloc_kxc_unpol],
        [lxc_unpol::lda_x_sloc_lxc_unpol],
        [exc_pol::lda_x_sloc_exc_pol],    [vxc_pol::lda_x_sloc_vxc_pol],
        [fxc_pol::lda_x_sloc_fxc_pol],    [kxc_pol::lda_x_sloc_kxc_pol],
        [lxc_pol::lda_x_sloc_lxc_pol],
        params = (1.67_f64, 0.3_f64)
    );
    Ok(())
}

fn launch_lda_x_yukawa(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_x_yukawa::*;
    // libxc default omega = 0.3
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_x_yukawa_exc_unpol], [vxc_unpol::lda_x_yukawa_vxc_unpol],
        [fxc_unpol::lda_x_yukawa_fxc_unpol], [kxc_unpol::lda_x_yukawa_kxc_unpol],
        [lxc_unpol::lda_x_yukawa_lxc_unpol],
        [exc_pol::lda_x_yukawa_exc_pol],    [vxc_pol::lda_x_yukawa_vxc_pol],
        [fxc_pol::lda_x_yukawa_fxc_pol],    [kxc_pol::lda_x_yukawa_kxc_pol],
        [lxc_pol::lda_x_yukawa_lxc_pol],
        params = (0.3_f64)
    );
    Ok(())
}

fn launch_lda_c_rpa(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_rpa::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_rpa_exc_unpol], [vxc_unpol::lda_c_rpa_vxc_unpol],
        [fxc_unpol::lda_c_rpa_fxc_unpol], [kxc_unpol::lda_c_rpa_kxc_unpol],
        [lxc_unpol::lda_c_rpa_lxc_unpol],
        [exc_pol::lda_c_rpa_exc_pol],    [vxc_pol::lda_c_rpa_vxc_pol],
        [fxc_pol::lda_c_rpa_fxc_pol],    [kxc_pol::lda_c_rpa_kxc_pol],
        [lxc_pol::lda_c_rpa_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_hl(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_hl::*;
    // libxc par_hl = {21.0, 21.0, 0.0225, 0.0225} mapped to
    // {hl_r_0, hl_r_1, hl_c_0, hl_c_1}; kernel param order is
    // (hl_c_0, hl_c_1, hl_r_0, hl_r_1).
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_hl_exc_unpol], [vxc_unpol::lda_c_hl_vxc_unpol],
        [fxc_unpol::lda_c_hl_fxc_unpol], [kxc_unpol::lda_c_hl_kxc_unpol],
        [lxc_unpol::lda_c_hl_lxc_unpol],
        [exc_pol::lda_c_hl_exc_pol],    [vxc_pol::lda_c_hl_vxc_pol],
        [fxc_pol::lda_c_hl_fxc_pol],    [kxc_pol::lda_c_hl_kxc_pol],
        [lxc_pol::lda_c_hl_lxc_pol],
        params = (0.0225_f64, 0.0225_f64, 21.0_f64, 21.0_f64)
    );
    Ok(())
}

fn launch_lda_c_vwn(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_vwn::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_vwn_exc_unpol], [vxc_unpol::lda_c_vwn_vxc_unpol],
        [fxc_unpol::lda_c_vwn_fxc_unpol], [kxc_unpol::lda_c_vwn_kxc_unpol],
        [lxc_unpol::lda_c_vwn_lxc_unpol],
        [exc_pol::lda_c_vwn_exc_pol],    [vxc_pol::lda_c_vwn_vxc_pol],
        [fxc_pol::lda_c_vwn_fxc_pol],    [kxc_pol::lda_c_vwn_kxc_pol],
        [lxc_pol::lda_c_vwn_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_vwn_rpa(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_vwn_rpa::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_vwn_rpa_exc_unpol], [vxc_unpol::lda_c_vwn_rpa_vxc_unpol],
        [fxc_unpol::lda_c_vwn_rpa_fxc_unpol], [kxc_unpol::lda_c_vwn_rpa_kxc_unpol],
        [lxc_unpol::lda_c_vwn_rpa_lxc_unpol],
        [exc_pol::lda_c_vwn_rpa_exc_pol],    [vxc_pol::lda_c_vwn_rpa_vxc_pol],
        [fxc_pol::lda_c_vwn_rpa_fxc_pol],    [kxc_pol::lda_c_vwn_rpa_kxc_pol],
        [lxc_pol::lda_c_vwn_rpa_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_vwn_1(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_vwn_1::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_vwn_1_exc_unpol], [vxc_unpol::lda_c_vwn_1_vxc_unpol],
        [fxc_unpol::lda_c_vwn_1_fxc_unpol], [kxc_unpol::lda_c_vwn_1_kxc_unpol],
        [lxc_unpol::lda_c_vwn_1_lxc_unpol],
        [exc_pol::lda_c_vwn_1_exc_pol],    [vxc_pol::lda_c_vwn_1_vxc_pol],
        [fxc_pol::lda_c_vwn_1_fxc_pol],    [kxc_pol::lda_c_vwn_1_kxc_pol],
        [lxc_pol::lda_c_vwn_1_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_vwn_2(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_vwn_2::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_vwn_2_exc_unpol], [vxc_unpol::lda_c_vwn_2_vxc_unpol],
        [fxc_unpol::lda_c_vwn_2_fxc_unpol], [kxc_unpol::lda_c_vwn_2_kxc_unpol],
        [lxc_unpol::lda_c_vwn_2_lxc_unpol],
        [exc_pol::lda_c_vwn_2_exc_pol],    [vxc_pol::lda_c_vwn_2_vxc_pol],
        [fxc_pol::lda_c_vwn_2_fxc_pol],    [kxc_pol::lda_c_vwn_2_kxc_pol],
        [lxc_pol::lda_c_vwn_2_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_vwn_3(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_vwn_3::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_vwn_3_exc_unpol], [vxc_unpol::lda_c_vwn_3_vxc_unpol],
        [fxc_unpol::lda_c_vwn_3_fxc_unpol], [kxc_unpol::lda_c_vwn_3_kxc_unpol],
        [lxc_unpol::lda_c_vwn_3_lxc_unpol],
        [exc_pol::lda_c_vwn_3_exc_pol],    [vxc_pol::lda_c_vwn_3_vxc_pol],
        [fxc_pol::lda_c_vwn_3_fxc_pol],    [kxc_pol::lda_c_vwn_3_kxc_pol],
        [lxc_pol::lda_c_vwn_3_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_vwn_4(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_vwn_4::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_vwn_4_exc_unpol], [vxc_unpol::lda_c_vwn_4_vxc_unpol],
        [fxc_unpol::lda_c_vwn_4_fxc_unpol], [kxc_unpol::lda_c_vwn_4_kxc_unpol],
        [lxc_unpol::lda_c_vwn_4_lxc_unpol],
        [exc_pol::lda_c_vwn_4_exc_pol],    [vxc_pol::lda_c_vwn_4_vxc_pol],
        [fxc_pol::lda_c_vwn_4_fxc_pol],    [kxc_pol::lda_c_vwn_4_kxc_pol],
        [lxc_pol::lda_c_vwn_4_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_pz(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_pz::*;
    // libxc par_pz indices = [gamma0, gamma1, beta1_0, beta1_1, beta2_0,
    // beta2_1, a0, a1, b0, b1, c0, c1, d0, d1].
    // Kernel parameter order is alphabetical:
    // (a_0, a_1, b_0, b_1, beta1_0, beta1_1, beta2_0, beta2_1,
    //  c_0, c_1, d_0, d_1, gamma_0, gamma_1).
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_pz_exc_unpol], [vxc_unpol::lda_c_pz_vxc_unpol],
        [fxc_unpol::lda_c_pz_fxc_unpol], [kxc_unpol::lda_c_pz_kxc_unpol],
        [lxc_unpol::lda_c_pz_lxc_unpol],
        [exc_pol::lda_c_pz_exc_pol],    [vxc_pol::lda_c_pz_vxc_pol],
        [fxc_pol::lda_c_pz_fxc_pol],    [kxc_pol::lda_c_pz_kxc_pol],
        [lxc_pol::lda_c_pz_lxc_pol],
        params = (
            0.0311_f64, 0.01555_f64,                 // a_0, a_1
            -0.048_f64, -0.0269_f64,                 // b_0, b_1
            1.0529_f64, 1.3981_f64,                  // beta1_0, beta1_1
            0.3334_f64, 0.2611_f64,                  // beta2_0, beta2_1
            0.0020_f64, 0.0007_f64,                  // c_0, c_1
            -0.0116_f64, -0.0048_f64,                // d_0, d_1
            -0.1423_f64, -0.0843_f64,                // gamma_0, gamma_1
        )
    );
    Ok(())
}

fn launch_lda_c_pw(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_pw::*;
    // libxc par_pw (id 12) — full 22-element array indexed [0..=21]:
    //   pp[0..=2], a[0..=2], alpha1[0..=2], beta1[0..=2], beta2[0..=2],
    //   beta3[0..=2], beta4[0..=2], fz20.
    // Unpol kernel takes indices (0, 2) for each [3]-array: 15 scalars.
    // Pol kernel takes all (0, 1, 2): 22 scalars.
    // Kernel parameter order in both is alphabetical:
    //   unpol: (a_0, a_2, alpha1_0, alpha1_2, beta1_0, beta1_2,
    //           beta2_0, beta2_2, beta3_0, beta3_2, beta4_0, beta4_2,
    //           fz20, pp_0, pp_2)
    //   pol:   (a_0, a_1, a_2, alpha1_0, alpha1_1, alpha1_2,
    //           beta1_0, beta1_1, beta1_2, beta2_0, beta2_1, beta2_2,
    //           beta3_0, beta3_1, beta3_2, beta4_0, beta4_1, beta4_2,
    //           fz20, pp_0, pp_1, pp_2)
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_pw_exc_unpol], [vxc_unpol::lda_c_pw_vxc_unpol],
        [fxc_unpol::lda_c_pw_fxc_unpol], [kxc_unpol::lda_c_pw_kxc_unpol],
        [lxc_unpol::lda_c_pw_lxc_unpol],
        [exc_pol::lda_c_pw_exc_pol],    [vxc_pol::lda_c_pw_vxc_pol],
        [fxc_pol::lda_c_pw_fxc_pol],    [kxc_pol::lda_c_pw_kxc_pol],
        [lxc_pol::lda_c_pw_lxc_pol],
        params_unpol = (
            0.031091_f64, 0.016887_f64,            // a_0, a_2
            0.21370_f64, 0.11125_f64,              // alpha1_0, alpha1_2
            7.5957_f64, 10.357_f64,                // beta1_0, beta1_2
            3.5876_f64, 3.6231_f64,                // beta2_0, beta2_2
            1.6382_f64, 0.88026_f64,               // beta3_0, beta3_2
            0.49294_f64, 0.49671_f64,              // beta4_0, beta4_2
            1.709921_f64,                           // fz20
            1.0_f64, 1.0_f64,                       // pp_0, pp_2
        ),
        params_pol = (
            0.031091_f64, 0.015545_f64, 0.016887_f64,    // a_0, a_1, a_2
            0.21370_f64, 0.20548_f64, 0.11125_f64,       // alpha1_0..2
            7.5957_f64, 14.1189_f64, 10.357_f64,         // beta1_0..2
            3.5876_f64, 6.1977_f64, 3.6231_f64,          // beta2_0..2
            1.6382_f64, 3.3662_f64, 0.88026_f64,         // beta3_0..2
            0.49294_f64, 0.62517_f64, 0.49671_f64,       // beta4_0..2
            1.709921_f64,                                 // fz20
            1.0_f64, 1.0_f64, 1.0_f64,                    // pp_0, pp_1, pp_2
        ),
    );
    Ok(())
}

fn launch_lda_c_wigner(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_wigner::*;
    // libxc val_wigner = {-0.44, 7.8} -> {a, b}
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_wigner_exc_unpol], [vxc_unpol::lda_c_wigner_vxc_unpol],
        [fxc_unpol::lda_c_wigner_fxc_unpol], [kxc_unpol::lda_c_wigner_kxc_unpol],
        [lxc_unpol::lda_c_wigner_lxc_unpol],
        [exc_pol::lda_c_wigner_exc_pol],    [vxc_pol::lda_c_wigner_vxc_pol],
        [fxc_pol::lda_c_wigner_fxc_pol],    [kxc_pol::lda_c_wigner_kxc_pol],
        [lxc_pol::lda_c_wigner_lxc_pol],
        params = (-0.44_f64, 7.8_f64)
    );
    Ok(())
}

fn launch_lda_c_rc04(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_rc04::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_rc04_exc_unpol], [vxc_unpol::lda_c_rc04_vxc_unpol],
        [fxc_unpol::lda_c_rc04_fxc_unpol], [kxc_unpol::lda_c_rc04_kxc_unpol],
        [lxc_unpol::lda_c_rc04_lxc_unpol],
        [exc_pol::lda_c_rc04_exc_pol],    [vxc_pol::lda_c_rc04_vxc_pol],
        [fxc_pol::lda_c_rc04_fxc_pol],    [kxc_pol::lda_c_rc04_kxc_pol],
        [lxc_pol::lda_c_rc04_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_2d_amgb(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_2d_amgb::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_2d_amgb_exc_unpol], [vxc_unpol::lda_c_2d_amgb_vxc_unpol],
        [fxc_unpol::lda_c_2d_amgb_fxc_unpol], [kxc_unpol::lda_c_2d_amgb_kxc_unpol],
        [lxc_unpol::lda_c_2d_amgb_lxc_unpol],
        [exc_pol::lda_c_2d_amgb_exc_pol],    [vxc_pol::lda_c_2d_amgb_vxc_pol],
        [fxc_pol::lda_c_2d_amgb_fxc_pol],    [kxc_pol::lda_c_2d_amgb_kxc_pol],
        [lxc_pol::lda_c_2d_amgb_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_2d_prm(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_2d_prm::*;
    // libxc default N_values = {2.0}; init computes c = M_PI / (2*(N-1)*q^2)
    // with q = 3.9274. For N=2 this gives c ≈ 0.10183793993557394.
    const PRM_C_DEFAULT: f64 = 0.101_837_939_935_573_94;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_2d_prm_exc_unpol], [vxc_unpol::lda_c_2d_prm_vxc_unpol],
        [fxc_unpol::lda_c_2d_prm_fxc_unpol], [kxc_unpol::lda_c_2d_prm_kxc_unpol],
        [lxc_unpol::lda_c_2d_prm_lxc_unpol],
        [exc_pol::lda_c_2d_prm_exc_pol],    [vxc_pol::lda_c_2d_prm_vxc_pol],
        [fxc_pol::lda_c_2d_prm_fxc_pol],    [kxc_pol::lda_c_2d_prm_kxc_pol],
        [lxc_pol::lda_c_2d_prm_lxc_pol],
        params = (PRM_C_DEFAULT)
    );
    Ok(())
}

fn launch_lda_c_1d_csc(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_1d_csc::*;
    // libxc default csc_values = {1, 1.0} -> interaction=1 (soft-Coulomb),
    // beta=1.0. csc_set_ext_params then chooses par_para[8] for both ppara
    // and pferro pointers, so unpol gets one 10-element array and pol gets
    // ferro+para = 20 scalars (ferro_0..9 followed by para_0..9, both
    // populated from par_para[8]).
    // par_para[8] = {18.40, 0.0, 7.501, 0.10185, 0.012827, 2.0, 3.0, 1.511, 0.258, 4.424}
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_1d_csc_exc_unpol], [vxc_unpol::lda_c_1d_csc_vxc_unpol],
        [fxc_unpol::lda_c_1d_csc_fxc_unpol], [kxc_unpol::lda_c_1d_csc_kxc_unpol],
        [lxc_unpol::lda_c_1d_csc_lxc_unpol],
        [exc_pol::lda_c_1d_csc_exc_pol],    [vxc_pol::lda_c_1d_csc_vxc_pol],
        [fxc_pol::lda_c_1d_csc_fxc_pol],    [kxc_pol::lda_c_1d_csc_kxc_pol],
        [lxc_pol::lda_c_1d_csc_lxc_pol],
        params_unpol = (
            // para_0..9
            18.40_f64, 0.0_f64, 7.501_f64, 0.10185_f64, 0.012827_f64,
            2.0_f64, 3.0_f64, 1.511_f64, 0.258_f64, 4.424_f64,
        ),
        params_pol = (
            // ferro_0..9 -- libxc csc_set_ext_params uses par_ferro[0]
            // for interaction=1, bb=1.0 (which is the default).
            5.24_f64, 0.0_f64, 1.568_f64, 0.12856_f64, 0.003201_f64,
            2.0_f64, 3.0_f64, 0.0538_f64, 1.56e-5_f64, 2.958_f64,
            // para_0..9 -- par_para[8] for the paramagnetic channel.
            18.40_f64, 0.0_f64, 7.501_f64, 0.10185_f64, 0.012827_f64,
            2.0_f64, 3.0_f64, 1.511_f64, 0.258_f64, 4.424_f64,
        ),
    );
    Ok(())
}

fn launch_lda_c_1d_loos(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_1d_loos::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_1d_loos_exc_unpol], [vxc_unpol::lda_c_1d_loos_vxc_unpol],
        [fxc_unpol::lda_c_1d_loos_fxc_unpol], [kxc_unpol::lda_c_1d_loos_kxc_unpol],
        [lxc_unpol::lda_c_1d_loos_lxc_unpol],
        [exc_pol::lda_c_1d_loos_exc_pol],    [vxc_pol::lda_c_1d_loos_vxc_pol],
        [fxc_pol::lda_c_1d_loos_fxc_pol],    [kxc_pol::lda_c_1d_loos_kxc_pol],
        [lxc_pol::lda_c_1d_loos_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_gk72(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_gk72::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_gk72_exc_unpol], [vxc_unpol::lda_c_gk72_vxc_unpol],
        [fxc_unpol::lda_c_gk72_fxc_unpol], [kxc_unpol::lda_c_gk72_kxc_unpol],
        [lxc_unpol::lda_c_gk72_lxc_unpol],
        [exc_pol::lda_c_gk72_exc_pol],    [vxc_pol::lda_c_gk72_vxc_pol],
        [fxc_pol::lda_c_gk72_fxc_pol],    [kxc_pol::lda_c_gk72_kxc_pol],
        [lxc_pol::lda_c_gk72_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_gombas(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_gombas::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_gombas_exc_unpol], [vxc_unpol::lda_c_gombas_vxc_unpol],
        [fxc_unpol::lda_c_gombas_fxc_unpol], [kxc_unpol::lda_c_gombas_kxc_unpol],
        [lxc_unpol::lda_c_gombas_lxc_unpol],
        [exc_pol::lda_c_gombas_exc_pol],    [vxc_pol::lda_c_gombas_vxc_pol],
        [fxc_pol::lda_c_gombas_fxc_pol],    [kxc_pol::lda_c_gombas_kxc_pol],
        [lxc_pol::lda_c_gombas_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_lp96(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_lp96::*;
    // libxc par_c_lp96 = {-0.0603, 0.0175, -0.00053} -> (C1, C2, C3)
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_lp96_exc_unpol], [vxc_unpol::lda_c_lp96_vxc_unpol],
        [fxc_unpol::lda_c_lp96_fxc_unpol], [kxc_unpol::lda_c_lp96_kxc_unpol],
        [lxc_unpol::lda_c_lp96_lxc_unpol],
        [exc_pol::lda_c_lp96_exc_pol],    [vxc_pol::lda_c_lp96_vxc_pol],
        [fxc_pol::lda_c_lp96_fxc_pol],    [kxc_pol::lda_c_lp96_kxc_pol],
        [lxc_pol::lda_c_lp96_lxc_pol],
        params = (-0.0603_f64, 0.0175_f64, -0.00053_f64)
    );
    Ok(())
}

fn launch_lda_c_ml1(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_ml1::*;
    // libxc par_ml1 = {0.2026, 0.084} -> (fc, q)
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_ml1_exc_unpol], [vxc_unpol::lda_c_ml1_vxc_unpol],
        [fxc_unpol::lda_c_ml1_fxc_unpol], [kxc_unpol::lda_c_ml1_kxc_unpol],
        [lxc_unpol::lda_c_ml1_lxc_unpol],
        [exc_pol::lda_c_ml1_exc_pol],    [vxc_pol::lda_c_ml1_vxc_pol],
        [fxc_pol::lda_c_ml1_fxc_pol],    [kxc_pol::lda_c_ml1_kxc_pol],
        [lxc_pol::lda_c_ml1_lxc_pol],
        params = (0.2026_f64, 0.084_f64)
    );
    Ok(())
}

fn launch_lda_c_w20(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_w20::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_w20_exc_unpol], [vxc_unpol::lda_c_w20_vxc_unpol],
        [fxc_unpol::lda_c_w20_fxc_unpol], [kxc_unpol::lda_c_w20_kxc_unpol],
        [lxc_unpol::lda_c_w20_lxc_unpol],
        [exc_pol::lda_c_w20_exc_pol],    [vxc_pol::lda_c_w20_vxc_pol],
        [fxc_pol::lda_c_w20_fxc_pol],    [kxc_pol::lda_c_w20_kxc_pol],
        [lxc_pol::lda_c_w20_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_c_chachiyo(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_chachiyo::*;
    // libxc par_chachiyo = {ap, bp, cp, af, bf, cf}; kernel param order
    // is alphabetical: (af, ap, bf, bp, cf, cp).
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_chachiyo_exc_unpol], [vxc_unpol::lda_c_chachiyo_vxc_unpol],
        [fxc_unpol::lda_c_chachiyo_fxc_unpol], [kxc_unpol::lda_c_chachiyo_kxc_unpol],
        [lxc_unpol::lda_c_chachiyo_lxc_unpol],
        [exc_pol::lda_c_chachiyo_exc_pol],    [vxc_pol::lda_c_chachiyo_vxc_pol],
        [fxc_pol::lda_c_chachiyo_fxc_pol],    [kxc_pol::lda_c_chachiyo_kxc_pol],
        [lxc_pol::lda_c_chachiyo_lxc_pol],
        params = (
            -0.007772675_f64,                       // af
            -0.01554535_f64,                        // ap
            27.4203609_f64,                         // bf
            20.4562557_f64,                         // bp
            27.4203609_f64,                         // cf
            20.4562557_f64,                         // cp
        )
    );
    Ok(())
}

fn launch_lda_c_chachiyo_mod(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_c_chachiyo_mod::*;
    // lda_c_chachiyo_mod uses the same par_chachiyo defaults.
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_c_chachiyo_mod_exc_unpol], [vxc_unpol::lda_c_chachiyo_mod_vxc_unpol],
        [fxc_unpol::lda_c_chachiyo_mod_fxc_unpol], [kxc_unpol::lda_c_chachiyo_mod_kxc_unpol],
        [lxc_unpol::lda_c_chachiyo_mod_lxc_unpol],
        [exc_pol::lda_c_chachiyo_mod_exc_pol],    [vxc_pol::lda_c_chachiyo_mod_vxc_pol],
        [fxc_pol::lda_c_chachiyo_mod_fxc_pol],    [kxc_pol::lda_c_chachiyo_mod_kxc_pol],
        [lxc_pol::lda_c_chachiyo_mod_lxc_pol],
        params = (
            -0.007772675_f64, -0.01554535_f64,
            27.4203609_f64, 20.4562557_f64,
            27.4203609_f64, 20.4562557_f64,
        )
    );
    Ok(())
}

fn launch_lda_k_tf(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_k_tf::*;
    // libxc lda_k_tf_init for XC_LDA_K_TF sets ax to:
    // 3/10 * pow(9*M_PI/4, 2/3) ≈ 1.104950565705860002098832079519635692942
    const TF_AX: f64 = 1.104_950_565_705_860_002_098_832_079_519_635_692_942;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_k_tf_exc_unpol], [vxc_unpol::lda_k_tf_vxc_unpol],
        [fxc_unpol::lda_k_tf_fxc_unpol], [kxc_unpol::lda_k_tf_kxc_unpol],
        [lxc_unpol::lda_k_tf_lxc_unpol],
        [exc_pol::lda_k_tf_exc_pol],    [vxc_pol::lda_k_tf_vxc_pol],
        [fxc_pol::lda_k_tf_fxc_pol],    [kxc_pol::lda_k_tf_kxc_pol],
        [lxc_pol::lda_k_tf_lxc_pol],
        params = (TF_AX)
    );
    Ok(())
}

fn launch_lda_k_zlp(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_k_zlp::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_k_zlp_exc_unpol], [vxc_unpol::lda_k_zlp_vxc_unpol],
        [fxc_unpol::lda_k_zlp_fxc_unpol], [kxc_unpol::lda_k_zlp_kxc_unpol],
        [lxc_unpol::lda_k_zlp_lxc_unpol],
        [exc_pol::lda_k_zlp_exc_pol],    [vxc_pol::lda_k_zlp_vxc_pol],
        [fxc_pol::lda_k_zlp_fxc_pol],    [kxc_pol::lda_k_zlp_kxc_pol],
        [lxc_pol::lda_k_zlp_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_xc_teter93(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_xc_teter93::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_xc_teter93_exc_unpol], [vxc_unpol::lda_xc_teter93_vxc_unpol],
        [fxc_unpol::lda_xc_teter93_fxc_unpol], [kxc_unpol::lda_xc_teter93_kxc_unpol],
        [lxc_unpol::lda_xc_teter93_lxc_unpol],
        [exc_pol::lda_xc_teter93_exc_pol],    [vxc_pol::lda_xc_teter93_vxc_pol],
        [fxc_pol::lda_xc_teter93_fxc_pol],    [kxc_pol::lda_xc_teter93_kxc_pol],
        [lxc_pol::lda_xc_teter93_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_xc_zlp(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_xc_zlp::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_xc_zlp_exc_unpol], [vxc_unpol::lda_xc_zlp_vxc_unpol],
        [fxc_unpol::lda_xc_zlp_fxc_unpol], [kxc_unpol::lda_xc_zlp_kxc_unpol],
        [lxc_unpol::lda_xc_zlp_lxc_unpol],
        [exc_pol::lda_xc_zlp_exc_pol],    [vxc_pol::lda_xc_zlp_vxc_pol],
        [fxc_pol::lda_xc_zlp_fxc_pol],    [kxc_pol::lda_xc_zlp_kxc_pol],
        [lxc_pol::lda_xc_zlp_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_xc_tih(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_xc_tih::*;
    eight_arm_vxc_only_dispatch!(
        ctx, order, spin,
        [vxc_unpol::lda_xc_tih_vxc_unpol], [fxc_unpol::lda_xc_tih_fxc_unpol],
        [kxc_unpol::lda_xc_tih_kxc_unpol], [lxc_unpol::lda_xc_tih_lxc_unpol],
        [vxc_pol::lda_xc_tih_vxc_pol],    [fxc_pol::lda_xc_tih_fxc_pol],
        [kxc_pol::lda_xc_tih_kxc_pol],    [lxc_pol::lda_xc_tih_lxc_pol],
        params = ()
    );
    Ok(())
}

fn launch_lda_xc_1d_ehwlrg(
    ctx: &LaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
    a1: f64,
    a2: f64,
    a3: f64,
    alpha: f64,
) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::lda_xc_1d_ehwlrg::*;
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::lda_xc_1d_ehwlrg_exc_unpol], [vxc_unpol::lda_xc_1d_ehwlrg_vxc_unpol],
        [fxc_unpol::lda_xc_1d_ehwlrg_fxc_unpol], [kxc_unpol::lda_xc_1d_ehwlrg_kxc_unpol],
        [lxc_unpol::lda_xc_1d_ehwlrg_lxc_unpol],
        [exc_pol::lda_xc_1d_ehwlrg_exc_pol],    [vxc_pol::lda_xc_1d_ehwlrg_vxc_pol],
        [fxc_pol::lda_xc_1d_ehwlrg_fxc_pol],    [kxc_pol::lda_xc_1d_ehwlrg_kxc_pol],
        [lxc_pol::lda_xc_1d_ehwlrg_lxc_pol],
        params = (a1, a2, a3, alpha)
    );
    Ok(())
}

fn launch_hyb_lda_xc_bn05(ctx: &LaunchCtx<'_>, order: DerivativeOrder, spin: Spin) -> Result<(), LibxcRsError> {
    use libxc_kernel_lda::hyb_lda_xc_bn05::*;
    // libxc par_bn05 = {1.0} -> param_hyb_omega_0
    ten_arm_dispatch!(
        ctx, order, spin,
        [exc_unpol::hyb_lda_xc_bn05_exc_unpol], [vxc_unpol::hyb_lda_xc_bn05_vxc_unpol],
        [fxc_unpol::hyb_lda_xc_bn05_fxc_unpol], [kxc_unpol::hyb_lda_xc_bn05_kxc_unpol],
        [lxc_unpol::hyb_lda_xc_bn05_lxc_unpol],
        [exc_pol::hyb_lda_xc_bn05_exc_pol],    [vxc_pol::hyb_lda_xc_bn05_vxc_pol],
        [fxc_pol::hyb_lda_xc_bn05_fxc_pol],    [kxc_pol::hyb_lda_xc_bn05_kxc_pol],
        [lxc_pol::hyb_lda_xc_bn05_lxc_pol],
        params = (1.0_f64)
    );
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::LdaInput;
    use crate::model::{DerivativeOrder, FunctionalId, LdaFunctional, Spin, Thresholds};
    use crate::output::LdaOutput;

    fn default_thresholds() -> Thresholds {
        Thresholds::default()
    }

    fn default_params() -> LdaFunctionalParams {
        LdaFunctionalParams::default()
    }

    #[test]
    fn test_exc_unpolarized_produces_negative_energy() {
        let rho = vec![0.1, 0.2, 0.5, 1.0];
        let np = 4;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Exc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }

    #[test]
    fn test_vxc_unpolarized_populates_both_zk_and_vrho() {
        let rho = vec![0.1, 0.5];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Vxc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        for &val in output.zk.unwrap().iter() {
            assert!(val < 0.0);
        }
        for &val in output.vrho.unwrap().iter() {
            assert!(val != 0.0);
        }
    }

    #[test]
    fn test_vxc_with_vrho_none_still_succeeds() {
        let rho = vec![0.1, 0.5];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Vxc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        for &val in output.zk.unwrap().iter() {
            assert!(val < 0.0);
        }
    }

    #[test]
    fn test_exc_polarized_routes_to_pol_kernel() {
        let np = 2;
        let rho = vec![0.1, 0.05, 0.2, 0.1];
        let input = LdaInput::new(&rho, np, Spin::Polarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Polarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Exc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        for &val in output.zk.unwrap().iter() {
            assert!(val < 0.0);
        }
    }

    #[test]
    fn test_vxc_polarized_vrho_has_2np_elements() {
        let np = 2;
        let rho = vec![0.1, 0.05, 0.2, 0.1];
        let input = LdaInput::new(&rho, np, Spin::Polarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np * 2];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), None, None, None, np, Spin::Polarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Vxc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        let vrho_result = output.vrho.unwrap();
        assert_eq!(vrho_result.len(), np * 2);
        for &val in vrho_result.iter() {
            assert!(val != 0.0);
        }
    }

    #[test]
    fn test_dispatch_zeros_output_buffers() {
        let rho = vec![0.1];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![999.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Exc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        assert!(zk_result[0] < 0.0);
        assert!(zk_result[0] > -2.0, "zk = {} suggests pre-fill contamination", zk_result[0]);
    }

    #[test]
    fn test_fxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Fxc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        assert!(output.zk.unwrap()[0] < 0.0);
        assert!(output.vrho.unwrap()[0] != 0.0);
        assert!(output.v2rho2.unwrap()[0] != 0.0);
    }

    #[test]
    fn test_kxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut v3rho3 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), Some(&mut v3rho3), None,
            np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Kxc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        assert!(output.zk.unwrap()[0] < 0.0);
        assert!(output.v3rho3.unwrap()[0] != 0.0);
    }

    #[test]
    fn test_lxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut v3rho3 = vec![0.0f64; np];
        let mut v4rho4 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), Some(&mut v3rho3), Some(&mut v4rho4),
            np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Lxc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        assert!(output.v4rho4.unwrap()[0] != 0.0);
    }

    #[test]
    fn test_lda_xc_tih_rejects_exc_request() {
        let rho = vec![0.1, 0.5];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        let err = dispatch_lda(LdaFunctional::LdaXcTih, &input, DerivativeOrder::Exc,
                               &mut output, &default_params(), &default_thresholds())
            .unwrap_err();
        match err {
            LibxcRsError::UnsupportedDerivativeOrder { id, order, .. } => {
                assert_eq!(id, FunctionalId::from_raw(599).unwrap());
                assert_eq!(order, DerivativeOrder::Exc);
            }
            other => panic!("expected UnsupportedDerivativeOrder, got {other:?}"),
        }
    }

    #[test]
    fn test_lda_xc_tih_vxc_succeeds() {
        let rho = vec![0.5, 1.0];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut vrho = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            None, Some(&mut vrho), None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaXcTih, &input, DerivativeOrder::Vxc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        let vrho_result = output.vrho.unwrap();
        for &val in vrho_result.iter() {
            assert!(val != 0.0, "vxc-only kernel should populate vrho");
        }
    }

    #[test]
    fn test_lda_c_vwn_exc_unpol() {
        let rho = vec![0.1, 0.5];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(LdaFunctional::LdaCVwn, &input, DerivativeOrder::Exc, &mut output,
                     &default_params(), &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for &val in zk_result.iter() {
            assert!(val < 0.0, "lda_c_vwn exc should be negative");
        }
    }
}
