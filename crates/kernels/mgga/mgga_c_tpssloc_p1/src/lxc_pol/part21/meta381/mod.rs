//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1840;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1841;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta381<F: Float>(t13931: F, t340: F, t343: F, t974: F, t10263: F, t10287: F, t10290: F, t10331: F, t10333: F, t10339: F, t10342: F, t10353: F, t13896: F, t13907: F, t13909: F, t13915: F, t1600: F, t2960: F, t4543: F, t973: F, t13804: F, t13845: F, t13894: F, t225: F, t68: F, t369: F, t1036: F, t4622: F, t3117: F, t4571: F, t248: F, t3051: F, t4347: F, t1041: F, t10370: F, t10372: F, t10377: F, t10381: F, t10385: F, t10390: F, t13750: F, t13751: F, t13758: F, t13762: F, t13767: F, t3070: F, t378: F, t4579: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13933, t13937) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1840::<F>(t13931, t340, t343, t974, t10263, t10287, t10290, t10331, t10333, t10339, t10342, t10353, t13896, t13907, t13909, t13915, t1600, t2960, t4543, t973);
        let (t13939, t13940, t13941, t13942, t13946, t13948, t13950) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1841::<F>(t13804, t13845, t13894, t13937, t225, t68, t369, t1036, t4622, t3117, t4571, t248, t3051, t4347);
        let (t13952, t13953) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1842::<F>(t1041, t13950, t10370, t10372, t10377, t10381, t10385, t10390, t13750, t13751, t13758, t13762, t13767, t13942, t13946, t13948, t3070, t378, t4579);
    (t13933, t13939, t13940, t13941, t13942, t13946, t13948, t13950, t13952, t13953)
}
