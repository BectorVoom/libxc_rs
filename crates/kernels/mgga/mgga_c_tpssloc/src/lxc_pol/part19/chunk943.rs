//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 943/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk943<F: Float>(t11521: F, t1177: F, t1178: F, t9258: F, t1176: F, t698: F, t1179: F, t1174: F, t3431: F, t3460: F, t3456: F, t135: F, t3439: F, t3442: F, t11499: F, t11505: F, t11510: F, t11514: F, t11518: F) -> (F, F, F, F) {
    let t11522 = t1177 * t11521;
    let t11525 = t1178 * t9258;
    let t11526 = t1177 * t11525;
    let t11529 = t698 * t1176;
    let t11530 = t11529 * t1179;
    let t11531 = t1174 * t11530;
    let t11533 = t3431 * t3460;
    let t11534 = t1174 * t11533;
    let t11536 = t3431 * t3456;
    let t11537 = t1174 * t11536;
    let t11539 = t135 * t3439;
    let t11540 = t11539 * t3442;
    let t11541 = t1174 * t11540;
    let t11543 = -0.83333333333333333332e-3 * t1174 * t11499 - 0.83333333333333333332e-3 * t1174 * t11505 - 0.24999999999999999999e-2 * t1174 * t11510 - 0.83333333333333333331e-3 * t11514 + 0.22222222222222222221e-2 * t1174 * t11518 - 0.16666666666666666666e-2 * t1174 * t11522 - 0.27777777777777777777e-3 * t1174 * t11526 + 0.18518518518518518518e-3 * t11531 - 0.27777777777777777777e-3 * t11534 - 0.55555555555555555554e-3 * t11537 + 0.37037037037037037036e-3 * t11541;
    (t11525, t11529, t11539, t11543)
}
