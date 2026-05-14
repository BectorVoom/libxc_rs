//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1003/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1003<F: Float>(t22724: F, t26344: F, t22643: F, t7691: F, t81195: F, t22573: F, t7684: F, t23993: F, t7435: F, t7432: F, t84241: F, t45844: F, t7025: F, t12571: F, t23966: F, t7428: F) -> (F, F, F, F, F, F, F, F) {
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91655 = t7684 * t22573;
    let t91905 = t7435 * t23993;
    let t91922 = t84241 * t7432;
    let t91954 = t45844 * t7025;
    let t91957 = t12571 * t23966;
    let t91996 = t7428 * t23993;
    (t91531, t91548, t91655, t91905, t91922, t91954, t91957, t91996)
}
