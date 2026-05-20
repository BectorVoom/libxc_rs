//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk670;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk671;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk672;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta98<F: Float>(t118: F, t776: F, t794: F, t2576: F, t59: F, t835: F, t154: F, t116: F, t206: F, t212: F, t225: F, t799: F, t2559: F, t222: F, t2563: F, t805: F, t68: F, t808: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2578, t2579, t2585) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk670::<F>(t118, t776, t794, t2576, t59, t835);
        let t2586 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk671::<F>(t154, t2585);
        let (t2588, t2590, t2597) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk672::<F>(t116, t206, t212, t2586, t225, t799);
        let (t2600, t2602, t2603, t2617) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk673::<F>(t154, t2559, t222, t2563, t805, t68, t808);
    (t2578, t2579, t2585, t2586, t2588, t2590, t2597, t2600, t2602, t2603, t2617)
}
