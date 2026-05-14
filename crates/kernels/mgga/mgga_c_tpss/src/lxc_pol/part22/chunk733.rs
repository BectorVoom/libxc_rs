//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 733/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk733<F: Float>(t1043: F, t4104: F, t1024: F, t1518: F, t2913: F, t1042: F, t2911: F, t2836: F, t2917: F, t4044: F, t4049: F, t4054: F, t4058: F, t1053: F, t1523: F, t1061: F, t1531: F) -> (F, F, F, F, F, F, F, F) {
    let t4105 = t4104 * t1043;
    let t4107 = 1.0 * t1024 * t4105;
    let t4108 = t1518 * t2913;
    let t4109 = t4108 * t1042;
    let t4111 = 0.16081979498692535067e2 * t2911 * t4109;
    let t4117 = t2917 - 0.57077777777777777777e-2 * t2836 - 0.57077777777777777777e-2 * t4044 - 0.11415555555555555555e-1 * t4049 + 0.34246666666666666666e-1 * t4054 + 0.17123333333333333333e-1 * t4058;
    let t4120 = t1523 * t1053;
    let t4125 = t1531 * t1061;
    (t4105, t4107, t4108, t4109, t4111, t4117, t4120, t4125)
}
