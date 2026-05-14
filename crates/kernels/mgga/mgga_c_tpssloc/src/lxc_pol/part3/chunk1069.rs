//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1069/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1069<F: Float>(t14731: F, t3440: F, t135: F, t5045: F, t1174: F, t1222: F, t4966: F, t1215: F, t1734: F, t1089: F, t475: F, t607: F, t3578: F, t4728: F, t1735: F, t3243: F) -> (F, F, F, F, F, F, F) {
    let t15686 = t3440 * t14731;
    let t15689 = t135 * t5045;
    let t15691 = t1174 * t15689 / 432.0;
    let t15699 = t4966 * t1222 / 2304.0;
    let t15700 = t1734 * t1215;
    let t15701 = t475 * t1089;
    let t15702 = t15701 * t607;
    let t15703 = t15700 * t15702;
    let t15704 = t3578 * t15703;
    let t15707 = t607 * t1215;
    let t15708 = t15707 * t475;
    let t15709 = t4728 * t15708;
    let t15710 = t3578 * t15709;
    let t15713 = t1735 * t3243;
    (t15686, t15691, t15699, t15704, t15708, t15710, t15713)
}
