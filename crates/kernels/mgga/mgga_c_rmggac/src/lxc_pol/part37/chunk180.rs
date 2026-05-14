//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 180/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk180<F: Float>(t1059: F, t158: F, t401: F, t402: F, t954: F, t957: F, t960: F, t964: F, t966: F, t969: F, t394: F, t166: F, t245: F, t363: F, t1054: F, t378: F) -> (F, F, F, F, F, F) {
    let t1060 = 1.0 / t1059;
    let t1061 = t158 * t1060;
    let t1062 = t401 * t401;
    let t1063 = t1062 * t402;
    let t1072 = -0.78438333333333333333e0 * t954 + 0.15687666666666666667e1 * t957 + 0.68863333333333333333e0 * t960 + 0.14025833333333333333e0 * t964 + 0.28051666666666666667e0 * t966 + 0.17365833333333333333e0 * t969;
    let t1073 = t1072 * t402;
    let t1076 = t394 * t394;
    let t1077 = 1.0 / t1076;
    let t1078 = t158 * t1077;
    let t1079 = t166 * t166;
    let t1080 = 1.0 / t1079;
    let t1081 = t1062 * t1080;
    let t1084 = t245 * t363;
    let t1087 = 0.35616666666666666666e-1 * t1054 * t1084 * t378;
    (t1061, t1063, t1073, t1078, t1081, t1087)
}
