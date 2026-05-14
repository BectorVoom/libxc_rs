//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1188/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1188<F: Float>(t605: F, t9259: F, t72: F, t79: F, t9240: F, t2235: F, t2251: F, t2307: F, t641: F, t9342: F, t531: F, t6995: F, t1983: F, t22596: F, t12012: F, t1390: F) -> (F, F, F, F, F, F, F) {
    let t83822 = t605 * t9259;
    let t83832 = t72 * t79 * t9240;
    let t83835 = t2235 * t2251;
    let t83840 = t72 * t641 * t2307;
    let t83846 = t72 * t79 * t9342;
    let t83859 = t531 * t6995;
    let t83862 = 18.0 * t1983 * t83859 * t22596;
    let t83863 = t1390 * t12012;
    (t83822, t83832, t83835, t83840, t83846, t83862, t83863)
}
