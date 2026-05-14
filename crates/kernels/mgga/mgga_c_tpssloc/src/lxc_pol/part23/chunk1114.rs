//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1114/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1114<F: Float>(t2904: F, t5769: F, t10632: F, t5790: F, t11094: F, t5946: F, t10189: F, t5842: F, t5836: F, t5838: F, t698: F, t973: F, t5844: F, t4509: F, t10224: F, t5824: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t60424 = t5769 * t2904;
    let t60722 = t5790 * t10632;
    let t60874 = t5946 * t11094;
    let t61189 = t10189 * t5842;
    let t61250 = t10189 * t5836;
    let t61310 = t973 * t698 * t5838;
    let t61313 = t973 * t698 * t5844;
    let t61322 = t4509 * t5836;
    let t61365 = t4509 * t5842;
    let t61408 = t973 * t10224 * t5824;
    (t60424, t60722, t60874, t61189, t61250, t61310, t61313, t61322, t61365, t61408)
}
