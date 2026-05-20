//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1639;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta379<F: Float>(t17766: F, t17798: F, t17852: F, t17873: F, t225: F, t68: F, t369: F, t10457: F, t248: F, t5677: F, t1041: F, t1044: F, t17187: F, t14084: F, t14085: F, t14117: F, t14508: F, t14511: F, t1622: F, t17734: F, t17738: F, t3048: F, t3117: F, t3130: F, t378: F, t4596: F, t4600: F, t4636: F, t4644: F, t5857: F, t5861: F, t973: F) -> (F, F, F, F, F, F, F, F) {
        let (t17875, t17876, t17877, t17878, t17884, t17885, t17890) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1639::<F>(t17766, t17798, t17852, t17873, t225, t68, t369, t10457, t248, t5677, t1041, t1044, t17187);
        let t17900 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1640::<F>(t1041, t14084, t14085, t14117, t14508, t14511, t1622, t17734, t17738, t17878, t17885, t17890, t3048, t3117, t3130, t378, t4596, t4600, t4636, t4644, t5857, t5861, t973);
    (t17875, t17876, t17877, t17878, t17884, t17885, t17890, t17900)
}
