//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1328/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1328<F: Float>(t2240: F, t2251: F, t835: F, t2250: F, t72: F, t79: F, t605: F, t9259: F, t9240: F, t2235: F, t2307: F, t641: F) -> (F, F, F, F, F, F, F) {
    let t83778 = t2240 * t2251;
    let t83803 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t835;
    let t83820 = t72 * t79 * t2250;
    let t83822 = t605 * t9259;
    let t83832 = t72 * t79 * t9240;
    let t83835 = t2235 * t2251;
    let t83840 = t72 * t641 * t2307;
    (t83778, t83803, t83820, t83822, t83832, t83835, t83840)
}
