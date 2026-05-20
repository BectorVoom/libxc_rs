//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk686;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk687;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk688;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta98<F: Float>(t40: F, t2244: F, t2250: F, t2433: F, t73: F, t197: F, zeta_threshold: F, t52: F, t76: F, t157: F, t182: F, t676: F, t724: F, t164: F, t723: F, t159: F, t730: F, t731: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2439, t2440) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk686::<F>(t40, t2244, t2250, t2433, t73, t197, zeta_threshold);
        let (t2447, t2448, t2450, t2454, t2458, t2459, t2460) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk687::<F>(t52, t2244, t2250, t2440, t76, t2439, t157, t182, t676, t724, t164, t723, t159, zeta_threshold);
        let t2461 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk688::<F>(t730);
        let (t2462, t2471) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk689::<F>(t2461, t731, t2388, t2391, t2394, t2398, t2400, t2403);
    (t2440, t2447, t2448, t2450, t2454, t2458, t2459, t2460, t2461, t2462, t2471)
}
