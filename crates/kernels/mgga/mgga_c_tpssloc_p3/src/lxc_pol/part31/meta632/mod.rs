//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1893;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta632<F: Float>(t1339: F, t19732: F, t6936: F, t22779: F, t28057: F, t6371: F, t80827: F, t28073: F, t80888: F, t26301: F, t7708: F, t91208: F, t26322: F, t91202: F, t20004: F, t26309: F, t19945: F, t19981: F, t22833: F, t19994: F, t221: F, t26284: F, t19631: F, t1998: F, t236: F, t6926: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97398, t97400, t97402, t97404, t97407) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1893::<F>(t1339, t19732, t6936, t22779, t28057, t6371, t80827, t28073, t80888, t26301, t7708, t91208);
        let (t97410, t97412, t97414, t97416, t97419, t97423) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1894::<F>(t26322, t7708, t91202, t20004, t26309, t19945, t19981, t22833, t19994, t221, t26284, t19631, t1998, t236, t6926);
    (t97398, t97400, t97402, t97404, t97407, t97410, t97412, t97414, t97416, t97419, t97423)
}
