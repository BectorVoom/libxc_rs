//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2200;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta703<F: Float>(t1774: F, t26135: F, t652: F, t26179: F, t7461: F, t25980: F, t7458: F, t1983: F, t28826: F, t31299: F, t1388: F, t6324: F, t26161: F, t91686: F, t26504: F, t7685: F, t22591: F, t28834: F, t19596: F, t6996: F, t24994: F, t7684: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t97865, t97869, t97871, t97874, t97875) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2200::<F>(t1774, t26135, t652, t26179, t7461, t25980, t7458, t1983, t28826, t31299, t1388, t6324);
        let (t97878, t97880, t97887, t97889, t97890) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2201::<F>(t26161, t91686, t97875, t26504, t7685, t1983, t22591, t28834, t19596, t6996, t24994, t7684);
    (t97865, t97869, t97871, t97874, t97878, t97880, t97887, t97889, t97890)
}
