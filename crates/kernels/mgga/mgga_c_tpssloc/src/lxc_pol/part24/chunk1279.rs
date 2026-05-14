//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1279/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1279<F: Float>(t1862: F, t2240: F, t2244: F, t607: F, t2250: F, t72: F, t79: F, t605: F, t9259: F, t39054: F, t6489: F, t39063: F, t9240: F, t2235: F, t2251: F, t2307: F, t641: F) -> (F, F, F, F, F, F, F, F, F) {
    let t83814 = t2240 * t2244 * t1862;
    let t83817 = t607 * t1862;
    let t83820 = t72 * t79 * t2250;
    let t83822 = t605 * t9259;
    let t83827 = t39054 * t6489;
    let t83830 = t39063 * t6489;
    let t83832 = t72 * t79 * t9240;
    let t83835 = t2235 * t2251;
    let t83840 = t72 * t641 * t2307;
    (t83814, t83817, t83820, t83822, t83827, t83830, t83832, t83835, t83840)
}
