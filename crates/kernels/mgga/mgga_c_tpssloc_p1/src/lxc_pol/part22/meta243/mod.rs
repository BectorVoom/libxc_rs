//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1337;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1338;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta243<F: Float>(t2967: F, t964: F, t340: F, t63: F, t344: F, t221: F, t339: F, t1032: F, t3082: F, t2393: F, t374: F, t376: F, t370: F, t3158: F, t3069: F, t3180: F, t3036: F, t67: F, t3067: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10333, t10335, t10337, t10339, t10372, t10375) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1337::<F>(t2967, t964, t340, t63, t344, t221, t339, t1032, t3082, t2393, t374, t376);
        let (t10377, t10381, t10383, t10385, t10390) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1338::<F>(t10375, t370, t3158, t964, t10335, t221, t339, t3069, t3180);
        let (t10401, t10402) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1339::<F>(t3036, t67, t3067);
    (t10333, t10337, t10339, t10372, t10375, t10377, t10381, t10383, t10385, t10390, t10401, t10402)
}
