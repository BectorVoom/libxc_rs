//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 181/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk181<F: Float>(t147: F, t362: F, t135: F, t376: F, t377: F, t954: F, t957: F, t960: F, t964: F, t966: F, t969: F, t364: F, t150: F, t245: F, t410: F, t171: F, t977: F) -> (F, F, F, F, F) {
    let t1088 = t362 * t147;
    let t1089 = 1.0 / t1088;
    let t1090 = t135 * t1089;
    let t1091 = t376 * t376;
    let t1092 = t1091 * t377;
    let t1094 = 2.0 * t1090 * t1092;
    let t1101 = -0.42198333333333333333e0 * t954 + 0.84396666666666666666e0 * t957 + 0.39862222222222222223e0 * t960 + 0.68258333333333333333e-1 * t964 + 0.13651666666666666667e0 * t966 + 0.13692777777777777778e0 * t969;
    let t1102 = t1101 * t377;
    let t1104 = 1.0 * t364 * t1102;
    let t1105 = t362 * t362;
    let t1106 = 1.0 / t1105;
    let t1107 = t135 * t1106;
    let t1108 = t150 * t150;
    let t1109 = 1.0 / t1108;
    let t1110 = t1091 * t1109;
    let t1112 = 0.16081979498692535067e2 * t1107 * t1110;
    let t1116 = t245 * t410;
    let t1120 = t171 * t977;
    (t1094, t1104, t1112, t1116, t1120)
}
