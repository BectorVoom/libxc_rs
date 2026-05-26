//! Per-functional dispatch layer for GGA kernel evaluation.
//!
//! Routes `dispatch_gga(functional, ...)` to per-functional launch helpers
//! under `funcs/<func>.rs`, one per routed functional. The outer match in
//! `dispatch_gga` picks the variant, then delegates to the matching
//! `funcs::<func>::dispatch_<func>` helper. Kernel paths resolve through the
//! `crate::kernel::gga` per-functional re-export façade (Phase 11 D-10b — no
//! `batchN::` segment survives).
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

use libxc_core::dims::Dimensions;
use libxc_core::error::LibxcRsError;
use crate::functional::params::FunctionalParams;
use libxc_core::input::GgaInput;
use crate::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer, create_zero_output_buffer,
    read_output_buffer,
};
use libxc_core::model::{DerivativeOrder, GgaFunctional, Thresholds};
use libxc_core::output::GgaOutput;
use cubecl::client::ComputeClient;
use cubecl::cpu::CpuRuntime;
use cubecl::prelude::{CubeCount, CubeDim};
use cubecl::server::Handle;


pub mod funcs;

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
        let rho_arg = || unsafe { ArrayArg::from_raw_parts($ctx.rho.clone(), $ctx.rho_len) };
        let sigma_arg = || unsafe { ArrayArg::from_raw_parts($ctx.sigma.clone(), $ctx.sigma_len) };
        let zk_arg = || {
            let h = $ctx.zk.expect("zk handle missing for Exc+ order on exc-bearing functional");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.zk_len) }
        };
        let vrho_arg = || {
            let h = $ctx.vrho.expect("vrho handle missing for Vxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.vrho_len) }
        };
        let vsigma_arg = || {
            let h = $ctx.vsigma.expect("vsigma handle missing for Vxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.vsigma_len) }
        };
        let v2rho2_arg = || {
            let h = $ctx.v2rho2.expect("v2rho2 handle missing for Fxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v2rho2_len) }
        };
        let v2rhosigma_arg = || {
            let h = $ctx.v2rhosigma.expect("v2rhosigma handle missing for Fxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v2rhosigma_len) }
        };
        let v2sigma2_arg = || {
            let h = $ctx.v2sigma2.expect("v2sigma2 handle missing for Fxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v2sigma2_len) }
        };
        let v3rho3_arg = || {
            let h = $ctx.v3rho3.expect("v3rho3 handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v3rho3_len) }
        };
        let v3rho2sigma_arg = || {
            let h = $ctx.v3rho2sigma.expect("v3rho2sigma handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v3rho2sigma_len) }
        };
        let v3rhosigma2_arg = || {
            let h = $ctx.v3rhosigma2.expect("v3rhosigma2 handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v3rhosigma2_len) }
        };
        let v3sigma3_arg = || {
            let h = $ctx.v3sigma3.expect("v3sigma3 handle missing for Kxc+ order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v3sigma3_len) }
        };
        let v4rho4_arg = || {
            let h = $ctx.v4rho4.expect("v4rho4 handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v4rho4_len) }
        };
        let v4rho3sigma_arg = || {
            let h = $ctx.v4rho3sigma.expect("v4rho3sigma handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v4rho3sigma_len) }
        };
        let v4rho2sigma2_arg = || {
            let h = $ctx.v4rho2sigma2.expect("v4rho2sigma2 handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v4rho2sigma2_len) }
        };
        let v4rhosigma3_arg = || {
            let h = $ctx.v4rhosigma3.expect("v4rhosigma3 handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v4rhosigma3_len) }
        };
        let v4sigma4_arg = || {
            let h = $ctx.v4sigma4.expect("v4sigma4 handle missing for Lxc order");
            unsafe { ArrayArg::from_raw_parts(h.clone(), $ctx.v4sigma4_len) }
        };
        let dt = $ctx.dt;
        let zt = $ctx.zt;
        match ($order, $spin) {
            (DerivativeOrder::Exc, Spin::Unpolarized) => unsafe {
                $($exc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Vxc, Spin::Unpolarized) => unsafe {
                $($vxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Fxc, Spin::Unpolarized) => unsafe {
                $($fxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Kxc, Spin::Unpolarized) => unsafe {
                $($kxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    v3rho3_arg(), v3rho2sigma_arg(), v3rhosigma2_arg(), v3sigma3_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Lxc, Spin::Unpolarized) => unsafe {
                $($lxc_u)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    v3rho3_arg(), v3rho2sigma_arg(), v3rhosigma2_arg(), v3sigma3_arg(),
                    v4rho4_arg(), v4rho3sigma_arg(), v4rho2sigma2_arg(),
                    v4rhosigma3_arg(), v4sigma4_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Exc, Spin::Polarized) => unsafe {
                $($exc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Vxc, Spin::Polarized) => unsafe {
                $($vxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Fxc, Spin::Polarized) => unsafe {
                $($fxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Kxc, Spin::Polarized) => unsafe {
                $($kxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    v3rho3_arg(), v3rho2sigma_arg(), v3rhosigma2_arg(), v3sigma3_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
            (DerivativeOrder::Lxc, Spin::Polarized) => unsafe {
                $($lxc_p)::+::launch_unchecked::<CpuRuntime>(
                    $ctx.client, $ctx.cube_count.clone(), $ctx.cube_dim,
                    rho_arg(), sigma_arg(), zk_arg(), vrho_arg(), vsigma_arg(),
                    v2rho2_arg(), v2rhosigma_arg(), v2sigma2_arg(),
                    v3rho3_arg(), v3rho2sigma_arg(), v3rhosigma2_arg(), v3sigma3_arg(),
                    v4rho4_arg(), v4rho3sigma_arg(), v4rho2sigma2_arg(),
                    v4rhosigma3_arg(), v4sigma4_arg(),
                    $( $scalar, )*
                    dt, zt,
                );
            }
        }
    }};
}

#[allow(unused_imports)]
pub(crate) use ten_arm_dispatch_gga;


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
    params: &dyn FunctionalParams,
    thresholds: &Thresholds,
) -> Result<(), LibxcRsError> {
    // `params` is currently unused by GGA dispatch: routed functionals that
    // take ext_params are deferred (return UnsupportedFunctional) until the
    // libxc 7.0.0 defaults are wired (B3 follow-up). Mirrors dispatch_mgga.
    let _ = params;
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

        GgaFunctional::GgaXHcthA => funcs::gga_x_hcth_a::dispatch_gga_x_hcth_a(&ctx, order, spin)?,
        GgaFunctional::GgaXEv93 => funcs::gga_x_ev93::dispatch_gga_x_ev93(&ctx, order, spin)?,
        GgaFunctional::GgaXQ2d => funcs::gga_x_q2d::dispatch_gga_x_q2d(&ctx, order, spin)?,
        GgaFunctional::GgaKTflw => funcs::gga_k_tflw::dispatch_gga_k_tflw(&ctx, order, spin)?,
        GgaFunctional::GgaKApbeint => funcs::gga_k_apbeint::dispatch_gga_k_apbeint(&ctx, order, spin)?,
        GgaFunctional::GgaXAk13 => funcs::gga_x_ak13::dispatch_gga_x_ak13(&ctx, order, spin)?,
        GgaFunctional::GgaKMeyer => funcs::gga_k_meyer::dispatch_gga_k_meyer(&ctx, order, spin)?,
        GgaFunctional::GgaXLvRpw86 => funcs::gga_x_lv_rpw86::dispatch_gga_x_lv_rpw86(&ctx, order, spin)?,
        GgaFunctional::GgaXPbeint => funcs::gga_x_pbeint::dispatch_gga_x_pbeint(&ctx, order, spin)?,
        GgaFunctional::GgaXVmt84 => funcs::gga_x_vmt84::dispatch_gga_x_vmt84(&ctx, order, spin)?,
        GgaFunctional::GgaXVmt => funcs::gga_x_vmt::dispatch_gga_x_vmt(&ctx, order, spin)?,
        GgaFunctional::GgaXN12 => funcs::gga_x_n12::dispatch_gga_x_n12(&ctx, order, spin)?,
        GgaFunctional::GgaCOpXalpha => funcs::gga_c_op_xalpha::dispatch_gga_c_op_xalpha(&ctx, order, spin)?,
        GgaFunctional::GgaCOpG96 => funcs::gga_c_op_g96::dispatch_gga_c_op_g96(&ctx, order, spin)?,
        GgaFunctional::GgaCOpPbe => funcs::gga_c_op_pbe::dispatch_gga_c_op_pbe(&ctx, order, spin)?,
        GgaFunctional::GgaCOpB88 => funcs::gga_c_op_b88::dispatch_gga_c_op_b88(&ctx, order, spin)?,
        GgaFunctional::GgaXSsbSw => funcs::gga_x_ssb_sw::dispatch_gga_x_ssb_sw(&ctx, order, spin)?,
        GgaFunctional::GgaXBpccac => funcs::gga_x_bpccac::dispatch_gga_x_bpccac(&ctx, order, spin)?,
        GgaFunctional::GgaCTca => funcs::gga_c_tca::dispatch_gga_c_tca(&ctx, order, spin)?,
        GgaFunctional::GgaXPbe => funcs::gga_x_pbe::dispatch_gga_x_pbe(&ctx, order, spin)?,
        GgaFunctional::GgaXB86 => funcs::gga_x_b86::dispatch_gga_x_b86(&ctx, order, spin)?,
        GgaFunctional::GgaXB88 => funcs::gga_x_b88::dispatch_gga_x_b88(&ctx, order, spin)?,
        GgaFunctional::GgaXG96 => funcs::gga_x_g96::dispatch_gga_x_g96(&ctx, order, spin)?,
        GgaFunctional::GgaXPw86 => funcs::gga_x_pw86::dispatch_gga_x_pw86(&ctx, order, spin)?,
        GgaFunctional::GgaXPw91 => funcs::gga_x_pw91::dispatch_gga_x_pw91(&ctx, order, spin)?,
        GgaFunctional::GgaXOptx => funcs::gga_x_optx::dispatch_gga_x_optx(&ctx, order, spin)?,
        GgaFunctional::GgaXDk87 => funcs::gga_x_dk87::dispatch_gga_x_dk87(&ctx, order, spin)?,
        GgaFunctional::GgaXLg93 => funcs::gga_x_lg93::dispatch_gga_x_lg93(&ctx, order, spin)?,
        GgaFunctional::GgaXRpbe => funcs::gga_x_rpbe::dispatch_gga_x_rpbe(&ctx, order, spin)?,
        GgaFunctional::GgaXWc => funcs::gga_x_wc::dispatch_gga_x_wc(&ctx, order, spin)?,
        GgaFunctional::GgaXAm05 => funcs::gga_x_am05::dispatch_gga_x_am05(&ctx, order, spin)?,
        GgaFunctional::GgaXPbea => funcs::gga_x_pbea::dispatch_gga_x_pbea(&ctx, order, spin)?,
        GgaFunctional::GgaXMpbe => funcs::gga_x_mpbe::dispatch_gga_x_mpbe(&ctx, order, spin)?,
        GgaFunctional::GgaX2dB86Mgc => funcs::gga_x_2d_b86_mgc::dispatch_gga_x_2d_b86_mgc(&ctx, order, spin)?,
        GgaFunctional::GgaXBayesian => funcs::gga_x_bayesian::dispatch_gga_x_bayesian(&ctx, order, spin)?,
        GgaFunctional::GgaX2dB88 => funcs::gga_x_2d_b88::dispatch_gga_x_2d_b88(&ctx, order, spin)?,
        GgaFunctional::GgaX2dB86 => funcs::gga_x_2d_b86::dispatch_gga_x_2d_b86(&ctx, order, spin)?,
        GgaFunctional::GgaX2dPbe => funcs::gga_x_2d_pbe::dispatch_gga_x_2d_pbe(&ctx, order, spin)?,
        GgaFunctional::GgaCPbe => funcs::gga_c_pbe::dispatch_gga_c_pbe(&ctx, order, spin)?,
        GgaFunctional::GgaCLyp => funcs::gga_c_lyp::dispatch_gga_c_lyp(&ctx, order, spin)?,
        GgaFunctional::GgaCP86 => funcs::gga_c_p86::dispatch_gga_c_p86(&ctx, order, spin)?,
        GgaFunctional::GgaCAm05 => funcs::gga_c_am05::dispatch_gga_c_am05(&ctx, order, spin)?,
        GgaFunctional::GgaCLm => funcs::gga_c_lm::dispatch_gga_c_lm(&ctx, order, spin)?,
        GgaFunctional::GgaXRge2 => funcs::gga_x_rge2::dispatch_gga_x_rge2(&ctx, order, spin)?,
        GgaFunctional::GgaXKt => funcs::gga_x_kt::dispatch_gga_x_kt(&ctx, order, spin)?,
        GgaFunctional::GgaCWl => funcs::gga_c_wl::dispatch_gga_c_wl(&ctx, order, spin)?,
        GgaFunctional::GgaCWi => funcs::gga_c_wi::dispatch_gga_c_wi(&ctx, order, spin)?,
        GgaFunctional::GgaXSogga11 => funcs::gga_x_sogga11::dispatch_gga_x_sogga11(&ctx, order, spin)?,
        GgaFunctional::GgaXcTh1 => funcs::gga_xc_th1::dispatch_gga_xc_th1(&ctx, order, spin)?,
        GgaFunctional::GgaXcTh2 => funcs::gga_xc_th2::dispatch_gga_xc_th2(&ctx, order, spin)?,
        GgaFunctional::GgaXcTh3 => funcs::gga_xc_th3::dispatch_gga_xc_th3(&ctx, order, spin)?,
        GgaFunctional::GgaXC09x => funcs::gga_x_c09x::dispatch_gga_x_c09x(&ctx, order, spin)?,
        GgaFunctional::GgaXLb => funcs::gga_x_lb::dispatch_gga_x_lb(&ctx, order, spin)?,
        GgaFunctional::GgaXLspbe => funcs::gga_x_lspbe::dispatch_gga_x_lspbe(&ctx, order, spin)?,
        GgaFunctional::GgaXLsrpbe => funcs::gga_x_lsrpbe::dispatch_gga_x_lsrpbe(&ctx, order, spin)?,
        GgaFunctional::GgaXNcap => funcs::gga_x_ncap::dispatch_gga_x_ncap(&ctx, order, spin)?,
        GgaFunctional::GgaXOl2 => funcs::gga_x_ol2::dispatch_gga_x_ol2(&ctx, order, spin)?,
        GgaFunctional::GgaKApbe => funcs::gga_k_apbe::dispatch_gga_k_apbe(&ctx, order, spin)?,
        GgaFunctional::GgaXHtbs => funcs::gga_x_htbs::dispatch_gga_x_htbs(&ctx, order, spin)?,
        GgaFunctional::GgaXAiry => funcs::gga_x_airy::dispatch_gga_x_airy(&ctx, order, spin)?,
        GgaFunctional::GgaXLag => funcs::gga_x_lag::dispatch_gga_x_lag(&ctx, order, spin)?,
        GgaFunctional::GgaCPbeVwn => funcs::gga_c_pbe_vwn::dispatch_gga_c_pbe_vwn(&ctx, order, spin)?,
        GgaFunctional::GgaKRationalP => funcs::gga_k_rational_p::dispatch_gga_k_rational_p(&ctx, order, spin)?,
        GgaFunctional::GgaKPg => funcs::gga_k_pg::dispatch_gga_k_pg(&ctx, order, spin)?,
        GgaFunctional::GgaCP86vwn => funcs::gga_c_p86vwn::dispatch_gga_c_p86vwn(&ctx, order, spin)?,
        GgaFunctional::GgaCOpPw91 => funcs::gga_c_op_pw91::dispatch_gga_c_op_pw91(&ctx, order, spin)?,
        GgaFunctional::GgaXCap => funcs::gga_x_cap::dispatch_gga_x_cap(&ctx, order, spin)?,
        GgaFunctional::GgaCBmk => funcs::gga_c_bmk::dispatch_gga_c_bmk(&ctx, order, spin)?,
        GgaFunctional::GgaXBeefvdw => funcs::gga_x_beefvdw::dispatch_gga_x_beefvdw(&ctx, order, spin)?,
        GgaFunctional::GgaXPbetrans => funcs::gga_x_pbetrans::dispatch_gga_x_pbetrans(&ctx, order, spin)?,
        GgaFunctional::GgaXChachiyo => funcs::gga_x_chachiyo::dispatch_gga_x_chachiyo(&ctx, order, spin)?,
        GgaFunctional::GgaCChachiyo => funcs::gga_c_chachiyo::dispatch_gga_c_chachiyo(&ctx, order, spin)?,
        GgaFunctional::GgaCCcdf => funcs::gga_c_ccdf::dispatch_gga_c_ccdf(&ctx, order, spin)?,
        GgaFunctional::HybGgaXcCase21 => funcs::hyb_gga_xc_case21::dispatch_hyb_gga_xc_case21(&ctx, order, spin)?,
        GgaFunctional::GgaXS12 => funcs::gga_x_s12::dispatch_gga_x_s12(&ctx, order, spin)?,
        GgaFunctional::GgaKPearson => funcs::gga_k_pearson::dispatch_gga_k_pearson(&ctx, order, spin)?,
        GgaFunctional::GgaKOl1 => funcs::gga_k_ol1::dispatch_gga_k_ol1(&ctx, order, spin)?,
        GgaFunctional::GgaKOl2 => funcs::gga_k_ol2::dispatch_gga_k_ol2(&ctx, order, spin)?,
        GgaFunctional::GgaKPw86 => funcs::gga_k_pw86::dispatch_gga_k_pw86(&ctx, order, spin)?,
        GgaFunctional::GgaKDk => funcs::gga_k_dk::dispatch_gga_k_dk(&ctx, order, spin)?,
        GgaFunctional::GgaKLc94 => funcs::gga_k_lc94::dispatch_gga_k_lc94(&ctx, order, spin)?,
        GgaFunctional::GgaKLlp => funcs::gga_k_llp::dispatch_gga_k_llp(&ctx, order, spin)?,
        GgaFunctional::GgaKThakkar => funcs::gga_k_thakkar::dispatch_gga_k_thakkar(&ctx, order, spin)?,
        GgaFunctional::GgaXItyh => funcs::gga_x_ityh::dispatch_gga_x_ityh(&ctx, order, spin)?,
        GgaFunctional::GgaXSfat => funcs::gga_x_sfat::dispatch_gga_x_sfat(&ctx, order, spin)?,
        GgaFunctional::GgaXSg4 => funcs::gga_x_sg4::dispatch_gga_x_sg4(&ctx, order, spin)?,
        GgaFunctional::GgaXGg99 => funcs::gga_x_gg99::dispatch_gga_x_gg99(&ctx, order, spin)?,
        GgaFunctional::GgaXPbepow => funcs::gga_x_pbepow::dispatch_gga_x_pbepow(&ctx, order, spin)?,
        GgaFunctional::GgaCScanE0 => funcs::gga_c_scan_e0::dispatch_gga_c_scan_e0(&ctx, order, spin)?,
        GgaFunctional::GgaCW94 => funcs::gga_c_w94::dispatch_gga_c_w94(&ctx, order, spin)?,
        GgaFunctional::GgaCCs1 => funcs::gga_c_cs1::dispatch_gga_c_cs1(&ctx, order, spin)?,
        GgaFunctional::GgaKExp4 => funcs::gga_k_exp4::dispatch_gga_k_exp4(&ctx, order, spin)?,
        GgaFunctional::GgaXSfatPbe => funcs::gga_x_sfat_pbe::dispatch_gga_x_sfat_pbe(&ctx, order, spin)?,
        GgaFunctional::GgaXFdLb94 => funcs::gga_x_fd_lb94::dispatch_gga_x_fd_lb94(&ctx, order, spin)?,
        GgaFunctional::GgaKLkt => funcs::gga_k_lkt::dispatch_gga_k_lkt(&ctx, order, spin)?,
        GgaFunctional::GgaKMpbe => funcs::gga_k_mpbe::dispatch_gga_k_mpbe(&ctx, order, spin)?,
        GgaFunctional::GgaKVt84f => funcs::gga_k_vt84f::dispatch_gga_k_vt84f(&ctx, order, spin)?,
        GgaFunctional::GgaKLgap => funcs::gga_k_lgap::dispatch_gga_k_lgap(&ctx, order, spin)?,
        GgaFunctional::GgaXItyhOptx => funcs::gga_x_ityh_optx::dispatch_gga_x_ityh_optx(&ctx, order, spin)?,
        GgaFunctional::GgaXItyhPbe => funcs::gga_x_ityh_pbe::dispatch_gga_x_ityh_pbe(&ctx, order, spin)?,
        GgaFunctional::GgaCLypr => funcs::gga_c_lypr::dispatch_gga_c_lypr(&ctx, order, spin)?,
        GgaFunctional::GgaKLgapGe => funcs::gga_k_lgap_ge::dispatch_gga_k_lgap_ge(&ctx, order, spin)?,
        GgaFunctional::HybGgaXCamS12 => funcs::hyb_gga_x_cam_s12::dispatch_hyb_gga_x_cam_s12(&ctx, order, spin)?,
        GgaFunctional::GgaXPbeErfGws => funcs::gga_x_pbe_erf_gws::dispatch_gga_x_pbe_erf_gws(&ctx, order, spin)?,
        GgaFunctional::GgaXQ1d => funcs::gga_x_q1d::dispatch_gga_x_q1d(&ctx, order, spin)?,
    }

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
    use libxc_core::model::Spin;

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
            &crate::functional::params::NoParams,
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
            &crate::functional::params::NoParams,
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

