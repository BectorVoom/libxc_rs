//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1224;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1225;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta328<F: Float>(t12985: F, t2586: F, t2570: F, t67: F, t792: F, t12984: F, t686: F, t776: F, t131: F, t9558: F, t205: F, t1489: F, t9541: F, t4126: F, t782: F, t4130: F, t2563: F, t4138: F, t4134: F, t9546: F, t118: F, t4119: F, t794: F, t2576: F, t225: F, t4266: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12986, t13002, t13005, t13010) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1224::<F>(t12985, t2586, t2570, t67, t792, t12984, t686, t776, t131, t9558, t205, t1489, t9541);
        let (t13014, t13020, t13022, t13027, t13042) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1225::<F>(t4126, t782, t4130, t2563, t4138, t4134, t9546, t118, t4119, t794, t2576, t225, t4266);
    (t12986, t13002, t13005, t13010, t13014, t13020, t13022, t13027, t13042)
}
