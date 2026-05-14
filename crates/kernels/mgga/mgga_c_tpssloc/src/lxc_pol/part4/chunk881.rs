//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 881/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk881<F: Float>(t15572: F, t3577: F, t3431: F, t4729: F, t1174: F, t1011: F, t15031: F, t1212: F, t1226: F, t4965: F, t11697: F, t4953: F, t1229: F, t3242: F, t13969: F, t4979: F) -> (F, F, F, F, F, F, F) {
    let t15574 = t3577 * t15572 / 3456.0;
    let t15578 = t3431 * t4729;
    let t15580 = t1174 * t15578 / 216.0;
    let t15590 = t15031 * t1011;
    let t15591 = t15590 * t1212;
    let t15594 = t4965 * t1226;
    let t15608 = t11697 * t4953;
    let t15610 = t3577 * t15608 / 3456.0;
    let t15615 = t1229 * t3242;
    let t15640 = t13969 * t4979;
    (t15574, t15580, t15591, t15594, t15610, t15615, t15640)
}
