//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 862/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk862<F: Float>(t15390: F, t18469: F, t18416: F, t4904: F, t18409: F, t4919: F, t18427: F, t11547: F, t20234: F, t11546: F, t1174: F, t15265: F, t1710: F, t1717: F, t18321: F, t22035: F, t22041: F, t22047: F, t22052: F, t22056: F, t22060: F, t22063: F, t3447: F, t4889: F, t6120: F, t6141: F, t6147: F) -> (F,) {
    let t22066 = t15390 * t18469;
    let t22069 = t18416 * t4904;
    let t22072 = t4919 * t18409;
    let t22075 = t4919 * t18427;
    let t22081 = t11547 * t20234;
    let t22082 = t11546 * t22081;
    let t22085 = -0.24444444444444444444e-1 * t18321 * t1717 + 0.66666666666666666666e-2 * t4889 * t6141 + 0.66666666666666666666e-2 * t4889 * t6147 - 0.83333333333333333332e-3 * t1174 * t22035 - 0.83333333333333333332e-3 * t1174 * t22041 - 0.81481481481481481478e-2 * t18321 * t1710 - 0.27777777777777777777e-3 * t1174 * t22047 - 0.24999999999999999999e-2 * t1174 * t22052 + 0.22222222222222222221e-2 * t1174 * t22056 - 0.16666666666666666666e-2 * t1174 * t22060 + 0.11111111111111111111e-2 * t3447 * t22063 - 0.11111111111111111111e-2 * t3447 * t22066 + 0.83333333333333333331e-3 * t3447 * t22069 + 0.83333333333333333331e-3 * t3447 * t22072 + 0.16666666666666666666e-2 * t3447 * t22075 + 0.14814814814814814814e-2 * t15265 - 0.29629629629629629629e-2 * t4889 * t6120 - 0.86419753086419753084e-3 * t1174 * t22082;
    (t22085,)
}
