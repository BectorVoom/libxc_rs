//! Match-based dispatch layer for LDA kernel evaluation.
//!
//! Routes evaluation requests to the correct CubeCL kernel function based on
//! functional ID, derivative order, and spin mode. All kernel launches go through
//! safe wrappers in `kernel::lda::launch_*` -- this module contains no raw kernel
//! launch calls (BUILD-04 compliant).

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::input::LdaInput;
use crate::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer,
    create_zero_output_buffer, read_output_buffer,
};
use crate::kernel::lda::launch_lda_x::BufArg;
use crate::model::{DerivativeOrder, Spin, Thresholds};
use crate::output::LdaOutput;

// Import all launch modules
use crate::kernel::lda::launch_lda_x;
use crate::kernel::lda::launch_lda_c_1d_csc;
use crate::kernel::lda::launch_lda_c_1d_loos;
use crate::kernel::lda::launch_lda_c_2d_amgb;
use crate::kernel::lda::launch_lda_c_2d_prm;
use crate::kernel::lda::launch_lda_c_chachiyo;
use crate::kernel::lda::launch_lda_c_chachiyo_mod;
use crate::kernel::lda::launch_lda_c_gk72;
use crate::kernel::lda::launch_lda_c_gombas;
use crate::kernel::lda::launch_lda_c_hl;
use crate::kernel::lda::launch_lda_c_lp96;
use crate::kernel::lda::launch_lda_c_ml1;
// use crate::kernel::lda::launch_lda_c_pk09;  // deferred: memory limits
use crate::kernel::lda::launch_lda_c_pmgb06;
use crate::kernel::lda::launch_lda_c_pw;
use crate::kernel::lda::launch_lda_c_pw_erf;
use crate::kernel::lda::launch_lda_c_pz;
use crate::kernel::lda::launch_lda_c_rc04;
use crate::kernel::lda::launch_lda_c_rpa;
use crate::kernel::lda::launch_lda_c_vwn;
use crate::kernel::lda::launch_lda_c_vwn_1;
use crate::kernel::lda::launch_lda_c_vwn_2;
use crate::kernel::lda::launch_lda_c_vwn_3;
use crate::kernel::lda::launch_lda_c_vwn_4;
use crate::kernel::lda::launch_lda_c_vwn_rpa;
use crate::kernel::lda::launch_lda_c_w20;
use crate::kernel::lda::launch_lda_c_wigner;
use crate::kernel::lda::launch_lda_k_gds08_worker;
use crate::kernel::lda::launch_lda_k_tf;
use crate::kernel::lda::launch_lda_k_zlp;
use crate::kernel::lda::launch_lda_x_2d;
use crate::kernel::lda::launch_lda_x_erf;
use crate::kernel::lda::launch_lda_x_rel;
use crate::kernel::lda::launch_lda_x_sloc;
use crate::kernel::lda::launch_lda_x_yukawa;
use crate::kernel::lda::launch_lda_xc_1d_ehwlrg;
// use crate::kernel::lda::launch_lda_xc_ksdt;  // deferred: memory limits
use crate::kernel::lda::launch_lda_xc_teter93;
use crate::kernel::lda::launch_lda_xc_zlp;
use crate::kernel::lda::launch_hyb_lda_xc_bn05;
use crate::kernel::lda::launch_lda_xc_tih;

/// Helper macro to dispatch a standard LDA functional (has exc through lxc).
///
/// Usage: `dispatch_lda_standard!(launch_module, client, cube_count, cube_dim,
///   rho_buf, zk_buf, vrho_buf, v2rho2_buf, v3rho3_buf, v4rho4_buf,
///   vrho_h, v2rho2_h, v3rho3_h, v4rho4_h,
///   vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
///   order, spin, map_launch_err, extra_params...)`
macro_rules! dispatch_lda_standard {
    ($mod:ident, $client:expr, $cc:expr, $cd:expr,
     $rho:expr, $zk:expr,
     $vrho_h:expr, $v2h:expr, $v3h:expr, $v4h:expr,
     $vrho_len:expr, $v2len:expr, $v3len:expr, $v4len:expr,
     $order:expr, $spin:expr, $map_err:expr,
     $($extra:expr),* $(,)?) => {
        match ($order, $spin) {
            (DerivativeOrder::Exc, Spin::Unpolarized) => {
                $mod::launch_exc_unpol(
                    $client, $cc, $cd, $rho, $zk,
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Vxc, Spin::Unpolarized) => {
                let vrho_h = $vrho_h.as_ref().unwrap();
                $mod::launch_vxc_unpol(
                    $client, $cc, $cd, $rho, $zk,
                    &BufArg::new(vrho_h, $vrho_len),
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Fxc, Spin::Unpolarized) => {
                let vrho_h = $vrho_h.as_ref().unwrap();
                let v2h = $v2h.as_ref().unwrap();
                $mod::launch_fxc_unpol(
                    $client, $cc, $cd, $rho, $zk,
                    &BufArg::new(vrho_h, $vrho_len),
                    &BufArg::new(v2h, $v2len),
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Kxc, Spin::Unpolarized) => {
                let vrho_h = $vrho_h.as_ref().unwrap();
                let v2h = $v2h.as_ref().unwrap();
                let v3h = $v3h.as_ref().unwrap();
                $mod::launch_kxc_unpol(
                    $client, $cc, $cd, $rho, $zk,
                    &BufArg::new(vrho_h, $vrho_len),
                    &BufArg::new(v2h, $v2len),
                    &BufArg::new(v3h, $v3len),
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Lxc, Spin::Unpolarized) => {
                let vrho_h = $vrho_h.as_ref().unwrap();
                let v2h = $v2h.as_ref().unwrap();
                let v3h = $v3h.as_ref().unwrap();
                let v4h = $v4h.as_ref().unwrap();
                $mod::launch_lxc_unpol(
                    $client, $cc, $cd, $rho, $zk,
                    &BufArg::new(vrho_h, $vrho_len),
                    &BufArg::new(v2h, $v2len),
                    &BufArg::new(v3h, $v3len),
                    &BufArg::new(v4h, $v4len),
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Exc, Spin::Polarized) => {
                $mod::launch_exc_pol(
                    $client, $cc, $cd, $rho, $zk,
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Vxc, Spin::Polarized) => {
                let vrho_h = $vrho_h.as_ref().unwrap();
                $mod::launch_vxc_pol(
                    $client, $cc, $cd, $rho, $zk,
                    &BufArg::new(vrho_h, $vrho_len),
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Fxc, Spin::Polarized) => {
                let vrho_h = $vrho_h.as_ref().unwrap();
                let v2h = $v2h.as_ref().unwrap();
                $mod::launch_fxc_pol(
                    $client, $cc, $cd, $rho, $zk,
                    &BufArg::new(vrho_h, $vrho_len),
                    &BufArg::new(v2h, $v2len),
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Kxc, Spin::Polarized) => {
                let vrho_h = $vrho_h.as_ref().unwrap();
                let v2h = $v2h.as_ref().unwrap();
                let v3h = $v3h.as_ref().unwrap();
                $mod::launch_kxc_pol(
                    $client, $cc, $cd, $rho, $zk,
                    &BufArg::new(vrho_h, $vrho_len),
                    &BufArg::new(v2h, $v2len),
                    &BufArg::new(v3h, $v3len),
                    $($extra,)*
                ).map_err($map_err)?;
            }
            (DerivativeOrder::Lxc, Spin::Polarized) => {
                let vrho_h = $vrho_h.as_ref().unwrap();
                let v2h = $v2h.as_ref().unwrap();
                let v3h = $v3h.as_ref().unwrap();
                let v4h = $v4h.as_ref().unwrap();
                $mod::launch_lxc_pol(
                    $client, $cc, $cd, $rho, $zk,
                    &BufArg::new(vrho_h, $vrho_len),
                    &BufArg::new(v2h, $v2len),
                    &BufArg::new(v3h, $v3len),
                    &BufArg::new(v4h, $v4len),
                    $($extra,)*
                ).map_err($map_err)?;
            }
        }
    };
}

/// Evaluate an LDA functional on the given input, writing results to output.
///
/// Routes to the correct kernel based on functional ID, derivative order, and
/// spin mode. Zeros caller output buffers before evaluation. Handles `None`
/// output fields by allocating dummy buffers the kernel writes to but whose
/// results are discarded.
///
/// # Arguments
/// * `func_id` - libxc functional ID (e.g., 1 for LDA_X, 7 for LDA_C_VWN)
/// * `input` - Validated LDA input bundle
/// * `order` - Maximum derivative order to compute
/// * `output` - Output bundle with optional buffers for each derivative level
/// * `params` - Functional-specific scalar parameters (empty for parameterless functionals)
/// * `thresholds` - Numerical thresholds for evaluation stability
///
/// # Errors
/// Returns `LibxcRsError` if evaluation fails.
pub fn dispatch_lda(
    func_id: u32,
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    params: &[f64],
    thresholds: &Thresholds,
) -> Result<(), LibxcRsError> {
    let np = input.np();
    let spin = input.spin();
    let dims = Dimensions::lda(spin);

    // Zero caller-provided output buffers (T-03-04 mitigation).
    if let Some(ref mut buf) = output.zk {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.vrho {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v2rho2 {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v3rho3 {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v4rho4 {
        buf.fill(0.0);
    }

    // Create CubeCL client and upload input
    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, input.rho());
    let rho_len = input.rho().len();

    // Create output handles for each derivative level up to `order`.
    let zk_len = np * dims.zk as usize;
    let zk_handle = create_zero_output_buffer(&client, zk_len);

    let vrho_len = np * dims.vrho as usize;
    let vrho_handle = if order >= DerivativeOrder::Vxc {
        Some(create_zero_output_buffer(&client, vrho_len))
    } else {
        None
    };

    let v2rho2_len = np * dims.v2rho2 as usize;
    let v2rho2_handle = if order >= DerivativeOrder::Fxc {
        Some(create_zero_output_buffer(&client, v2rho2_len))
    } else {
        None
    };

    let v3rho3_len = np * dims.v3rho3 as usize;
    let v3rho3_handle = if order >= DerivativeOrder::Kxc {
        Some(create_zero_output_buffer(&client, v3rho3_len))
    } else {
        None
    };

    let v4rho4_len = np * dims.v4rho4 as usize;
    let v4rho4_handle = if order >= DerivativeOrder::Lxc {
        Some(create_zero_output_buffer(&client, v4rho4_len))
    } else {
        None
    };

    let (cube_count, cube_dim) = calculate_launch_config(np);

    let rho_buf = BufArg::new(&rho_handle, rho_len);
    let zk_buf = BufArg::new(&zk_handle, zk_len);

    let map_launch_err = |e: Box<dyn std::error::Error>| LibxcRsError::KernelLaunchFailed {
        reason: e.to_string(),
    };

    // Two-level dispatch: first by func_id, then by (order, spin).
    // The launch wrapper function names follow the pattern:
    //   launch_{name}::launch_{name}_{order}_{spin}
    // but the macro expects shortened names without the prefix.
    match func_id {
        // XC_LDA_X = 1 (Slater exchange)
        // Also: XC_LDA_C_XALPHA = 6, XC_LDA_X_RAE = 549
        1 | 6 | 549 => {
            let alpha = if params.is_empty() { 1.0 } else { params[0] };
            match (order, spin) {
                (DerivativeOrder::Exc, Spin::Unpolarized) => {
                    launch_lda_x::launch_lda_x_exc_unpol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Vxc, Spin::Unpolarized) => {
                    let vrho_h = vrho_handle.as_ref().unwrap();
                    launch_lda_x::launch_lda_x_vxc_unpol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        &BufArg::new(vrho_h, vrho_len),
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Fxc, Spin::Unpolarized) => {
                    let vrho_h = vrho_handle.as_ref().unwrap();
                    let v2rho2_h = v2rho2_handle.as_ref().unwrap();
                    launch_lda_x::launch_lda_x_fxc_unpol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        &BufArg::new(vrho_h, vrho_len),
                        &BufArg::new(v2rho2_h, v2rho2_len),
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Kxc, Spin::Unpolarized) => {
                    let vrho_h = vrho_handle.as_ref().unwrap();
                    let v2rho2_h = v2rho2_handle.as_ref().unwrap();
                    let v3rho3_h = v3rho3_handle.as_ref().unwrap();
                    launch_lda_x::launch_lda_x_kxc_unpol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        &BufArg::new(vrho_h, vrho_len),
                        &BufArg::new(v2rho2_h, v2rho2_len),
                        &BufArg::new(v3rho3_h, v3rho3_len),
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Lxc, Spin::Unpolarized) => {
                    let vrho_h = vrho_handle.as_ref().unwrap();
                    let v2rho2_h = v2rho2_handle.as_ref().unwrap();
                    let v3rho3_h = v3rho3_handle.as_ref().unwrap();
                    let v4rho4_h = v4rho4_handle.as_ref().unwrap();
                    launch_lda_x::launch_lda_x_lxc_unpol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        &BufArg::new(vrho_h, vrho_len),
                        &BufArg::new(v2rho2_h, v2rho2_len),
                        &BufArg::new(v3rho3_h, v3rho3_len),
                        &BufArg::new(v4rho4_h, v4rho4_len),
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Exc, Spin::Polarized) => {
                    launch_lda_x::launch_lda_x_exc_pol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Vxc, Spin::Polarized) => {
                    let vrho_h = vrho_handle.as_ref().unwrap();
                    launch_lda_x::launch_lda_x_vxc_pol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        &BufArg::new(vrho_h, vrho_len),
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Fxc, Spin::Polarized) => {
                    let vrho_h = vrho_handle.as_ref().unwrap();
                    let v2rho2_h = v2rho2_handle.as_ref().unwrap();
                    launch_lda_x::launch_lda_x_fxc_pol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        &BufArg::new(vrho_h, vrho_len),
                        &BufArg::new(v2rho2_h, v2rho2_len),
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Kxc, Spin::Polarized) => {
                    let vrho_h = vrho_handle.as_ref().unwrap();
                    let v2rho2_h = v2rho2_handle.as_ref().unwrap();
                    let v3rho3_h = v3rho3_handle.as_ref().unwrap();
                    launch_lda_x::launch_lda_x_kxc_pol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        &BufArg::new(vrho_h, vrho_len),
                        &BufArg::new(v2rho2_h, v2rho2_len),
                        &BufArg::new(v3rho3_h, v3rho3_len),
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
                (DerivativeOrder::Lxc, Spin::Polarized) => {
                    let vrho_h = vrho_handle.as_ref().unwrap();
                    let v2rho2_h = v2rho2_handle.as_ref().unwrap();
                    let v3rho3_h = v3rho3_handle.as_ref().unwrap();
                    let v4rho4_h = v4rho4_handle.as_ref().unwrap();
                    launch_lda_x::launch_lda_x_lxc_pol(
                        &client, cube_count, cube_dim,
                        &rho_buf, &zk_buf,
                        &BufArg::new(vrho_h, vrho_len),
                        &BufArg::new(v2rho2_h, v2rho2_len),
                        &BufArg::new(v3rho3_h, v3rho3_len),
                        &BufArg::new(v4rho4_h, v4rho4_len),
                        alpha, thresholds.density, thresholds.zeta,
                    ).map_err(map_launch_err)?;
                }
            }
        }

        // XC_LDA_C_WIGNER = 2
        // Also: XC_LDA_C_MCWEENY = 551, XC_LDA_C_BR78 = 552,
        //       XC_LDA_C_OW_LYP = 573, XC_LDA_C_OW = 574
        2 | 551 | 552 | 573 | 574 => {
            dispatch_lda_standard!(launch_lda_c_wigner, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_RPA = 3
        3 => {
            dispatch_lda_standard!(launch_lda_c_rpa, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_HL = 4
        // Also: XC_LDA_C_VBH = 17
        4 | 17 => {
            dispatch_lda_standard!(launch_lda_c_hl, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], params[3],
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_GL = 5 -- uses same kernel as lda_c_vwn
        // (GL is VWN with different constants, but shares kernel structure)

        // XC_LDA_C_VWN = 7
        7 => {
            dispatch_lda_standard!(launch_lda_c_vwn, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_VWN_RPA = 8
        8 => {
            dispatch_lda_standard!(launch_lda_c_vwn_rpa, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_PZ = 9
        // Also: XC_LDA_C_PZ_MOD = 10, XC_LDA_C_OB_PZ = 11
        9 | 10 | 11 => {
            dispatch_lda_standard!(launch_lda_c_pz, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], params[3], params[4], params[5],
                params[6], params[7], params[8], params[9], params[10], params[11],
                params[12], params[13],
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_PW = 12
        // Also: XC_LDA_C_PW_MOD = 13, XC_LDA_C_OB_PW = 14, XC_LDA_C_PW_RPA = 25
        12 | 13 | 14 | 25 => {
            dispatch_lda_standard!(launch_lda_c_pw, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], params[3], params[4], params[5],
                params[6], params[7], params[8], params[9], params[10], params[11],
                params[12], params[13], params[14],
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_2D_AMGB = 15
        15 => {
            dispatch_lda_standard!(launch_lda_c_2d_amgb, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_2D_PRM = 16
        16 => {
            dispatch_lda_standard!(launch_lda_c_2d_prm, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_1D_CSC = 18
        18 => {
            dispatch_lda_standard!(launch_lda_c_1d_csc, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], params[3], params[4],
                params[5], params[6], params[7], params[8], params[9],
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_X_2D = 19
        19 => {
            dispatch_lda_standard!(launch_lda_x_2d, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_XC_TETER93 = 20
        20 => {
            dispatch_lda_standard!(launch_lda_xc_teter93, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_ML1 = 22
        // Also: XC_LDA_C_ML2 = 23
        22 | 23 => {
            dispatch_lda_standard!(launch_lda_c_ml1, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_GOMBAS = 24
        24 => {
            dispatch_lda_standard!(launch_lda_c_gombas, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_1D_LOOS = 26
        26 => {
            dispatch_lda_standard!(launch_lda_c_1d_loos, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_RC04 = 27
        27 => {
            dispatch_lda_standard!(launch_lda_c_rc04, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_VWN_1 = 28
        28 => {
            dispatch_lda_standard!(launch_lda_c_vwn_1, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_VWN_2 = 29
        29 => {
            dispatch_lda_standard!(launch_lda_c_vwn_2, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_VWN_3 = 30
        30 => {
            dispatch_lda_standard!(launch_lda_c_vwn_3, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_VWN_4 = 31
        31 => {
            dispatch_lda_standard!(launch_lda_c_vwn_4, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_XC_ZLP = 43
        43 => {
            dispatch_lda_standard!(launch_lda_xc_zlp, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_K_TF = 50
        50 => {
            dispatch_lda_standard!(launch_lda_k_tf, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_K_LP = 51 -- uses lda_k_tf kernel with different params
        51 => {
            dispatch_lda_standard!(launch_lda_k_tf, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_XC_KSDT = 259 -- deferred: exceeds compiler memory limits
        // Also: XC_LDA_XC_CORRKSDT = 318, XC_LDA_XC_GDSMFB = 577
        259 | 318 | 577 => {
            return Err(LibxcRsError::UnsupportedFunctional { func_id });
        }

        // XC_LDA_C_CHACHIYO = 287
        // Also: XC_LDA_C_KARASIEV = 579
        287 | 579 => {
            dispatch_lda_standard!(launch_lda_c_chachiyo, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], params[3], params[4], params[5],
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_LP96 = 289
        // Also: XC_LDA_K_LP96 = 580
        289 | 580 => {
            dispatch_lda_standard!(launch_lda_c_lp96, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_CHACHIYO_MOD = 307
        // Also: XC_LDA_C_KARASIEV_MOD = 308
        307 | 308 => {
            dispatch_lda_standard!(launch_lda_c_chachiyo_mod, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], params[3], params[4], params[5],
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_W20 = 317
        317 => {
            dispatch_lda_standard!(launch_lda_c_w20, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_X_REL = 532
        532 => {
            dispatch_lda_standard!(launch_lda_x_rel, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_XC_1D_EHWLRG_1 = 536, _2 = 537, _3 = 538
        536 | 537 | 538 => {
            dispatch_lda_standard!(launch_lda_xc_1d_ehwlrg, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], params[3],
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_X_ERF = 546
        // Also: XC_HYB_LDA_X_ERF = 653
        546 | 653 => {
            dispatch_lda_standard!(launch_lda_x_erf, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_XC_LP_A = 547, XC_LDA_XC_LP_B = 548
        // These use lda_c_lp96 kernel with different params
        547 | 548 => {
            dispatch_lda_standard!(launch_lda_c_lp96, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_K_ZLP = 550
        550 => {
            dispatch_lda_standard!(launch_lda_k_zlp, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_PK09 = 554 -- deferred: exceeds compiler memory limits
        554 => {
            return Err(LibxcRsError::UnsupportedFunctional { func_id });
        }

        // XC_LDA_C_GK72 = 578
        578 => {
            dispatch_lda_standard!(launch_lda_c_gk72, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_HYB_LDA_XC_BN05 = 588
        588 => {
            dispatch_lda_standard!(launch_hyb_lda_xc_bn05, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_PMGB06 = 590
        590 => {
            dispatch_lda_standard!(launch_lda_c_pmgb06, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_XC_TIH = 599
        599 => {
            // tih has no exc, only vxc through lxc
            if order == DerivativeOrder::Exc {
                return Err(LibxcRsError::FuncOrderNotSupported {
                    func_id,
                    order: "exc".to_string(),
                });
            }
            dispatch_lda_standard!(launch_lda_xc_tih, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_X_YUKAWA = 641
        641 => {
            dispatch_lda_standard!(launch_lda_x_yukawa, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_PW_ERF = 654
        654 => {
            dispatch_lda_standard!(launch_lda_c_pw_erf, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_C_UPW92 = 683, XC_LDA_C_RPW92 = 684
        // These use lda_c_pw kernel with different params
        683 | 684 => {
            dispatch_lda_standard!(launch_lda_c_pw, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], params[3], params[4], params[5],
                params[6], params[7], params[8], params[9], params[10], params[11],
                params[12], params[13], params[14],
                thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_X_SLOC = 692
        692 => {
            dispatch_lda_standard!(launch_lda_x_sloc, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], thresholds.density, thresholds.zeta,
            );
        }

        // XC_LDA_K_GDS08 uses lda_k_gds08_worker kernel
        // Multiple IDs share this: look up by range
        // For now, use a catch-all for gds08 variants
        _ if is_lda_k_gds08(func_id) => {
            dispatch_lda_standard!(launch_lda_k_gds08_worker, &client, cube_count, cube_dim,
                &rho_buf, &zk_buf,
                &vrho_handle, &v2rho2_handle, &v3rho3_handle, &v4rho4_handle,
                vrho_len, v2rho2_len, v3rho3_len, v4rho4_len,
                order, spin, map_launch_err,
                params[0], params[1], params[2], thresholds.density, thresholds.zeta,
            );
        }

        _ => {
            return Err(LibxcRsError::UnsupportedFunctional { func_id });
        }
    }

    // Read back results from CubeCL buffers into caller-provided output slices.
    if let Some(ref mut buf) = output.zk {
        let result = read_output_buffer(&client, zk_handle, zk_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "zk",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.vrho, vrho_handle) {
        let result = read_output_buffer(&client, h, vrho_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "vrho",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v2rho2, v2rho2_handle) {
        let result = read_output_buffer(&client, h, v2rho2_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v2rho2",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v3rho3, v3rho3_handle) {
        let result = read_output_buffer(&client, h, v3rho3_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v3rho3",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v4rho4, v4rho4_handle) {
        let result = read_output_buffer(&client, h, v4rho4_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v4rho4",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }

    Ok(())
}

/// Check if a functional ID corresponds to a GDS08 kinetic energy worker variant.
fn is_lda_k_gds08(_func_id: u32) -> bool {
    // GDS08 worker IDs would be looked up from a registry.
    // For now return false; specific IDs will be added as needed.
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::LdaInput;
    use crate::model::{DerivativeOrder, Spin, Thresholds};
    use crate::output::LdaOutput;

    fn default_thresholds() -> Thresholds {
        Thresholds::default()
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

        dispatch_lda(1, &input, DerivativeOrder::Exc, &mut output, &[1.0], &default_thresholds()).unwrap();

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

        dispatch_lda(1, &input, DerivativeOrder::Vxc, &mut output, &[1.0], &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
        let vrho_result = output.vrho.unwrap();
        for (i, &val) in vrho_result.iter().enumerate() {
            assert!(val != 0.0, "vrho[{i}] should be non-zero");
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

        dispatch_lda(1, &input, DerivativeOrder::Vxc, &mut output, &[1.0], &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
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

        dispatch_lda(1, &input, DerivativeOrder::Exc, &mut output, &[1.0], &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative (polarized)");
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

        dispatch_lda(1, &input, DerivativeOrder::Vxc, &mut output, &[1.0], &default_thresholds()).unwrap();

        let vrho_result = output.vrho.unwrap();
        assert_eq!(vrho_result.len(), np * 2);
        for (i, &val) in vrho_result.iter().enumerate() {
            assert!(val != 0.0, "vrho[{i}] should be non-zero (polarized)");
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

        dispatch_lda(1, &input, DerivativeOrder::Exc, &mut output, &[1.0], &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        assert!(zk_result[0] < 0.0, "zk should be negative, not contaminated by pre-fill");
        assert!(zk_result[0] > -2.0, "zk = {} seems too negative (pre-fill contamination?)", zk_result[0]);
    }

    #[test]
    fn test_unsupported_functional_returns_error() {
        let rho = vec![0.1];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        let result = dispatch_lda(99999, &input, DerivativeOrder::Exc, &mut output, &[], &default_thresholds());
        assert!(result.is_err());
    }
}
