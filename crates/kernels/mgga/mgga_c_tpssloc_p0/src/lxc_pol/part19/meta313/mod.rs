//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1113;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1114;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta313<F: Float>(t1291: F, t39358: F, t2369: F, t9720: F, t9843: F, t1294: F, t3814: F, t9874: F, t1307: F, t3914: F, t2411: F, t2414: F, t39246: F, t3691: F, t9494: F, t2508: F, t2511: F, t3918: F, t39335: F, t39338: F, t39340: F, t39342: F, t39346: F, t39349: F, t39350: F, t39356: F, t6999: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39360, t39362, t39364, t39366, t39367, t39373) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1113::<F>(t1291, t39358, t2369, t9720, t9843, t1294, t3814, t9874, t1307, t3914, t2411, t2414, t39246);
        let (t39375, t39377, t39378) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1114::<F>(t3691, t9494, t2508, t2369);
        let (t39381, t39382, t39384, t39385) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1115::<F>(t2511, t39377, t39378, t1294, t1307, t3918, t39335, t39338, t39340, t39342, t39346, t39349, t39350, t39356, t39360, t39364, t39366, t39367, t39373, t39375, t6999);
    (t39360, t39362, t39364, t39366, t39373, t39375, t39377, t39378, t39381, t39382, t39384, t39385)
}
