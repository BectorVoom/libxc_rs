//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1045/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1045<F: Float>(t2250: F, t72: F, t79: F, t605: F, t9259: F, t9240: F, t2235: F, t2251: F, t2307: F, t641: F, t9342: F, t12012: F, t1390: F, t22573: F, t6875: F, t191: F, t192: F, t9419: F) -> (F, F, F, F, F, F, F, F, F) {
    let t83820 = t72 * t79 * t2250;
    let t83822 = t605 * t9259;
    let t83832 = t72 * t79 * t9240;
    let t83835 = t2235 * t2251;
    let t83840 = t72 * t641 * t2307;
    let t83846 = t72 * t79 * t9342;
    let t83863 = t1390 * t12012;
    let t83886 = t6875 * t22573;
    let t83904 = t9419 * t191 * t192;
    (t83820, t83822, t83832, t83835, t83840, t83846, t83863, t83886, t83904)
}
