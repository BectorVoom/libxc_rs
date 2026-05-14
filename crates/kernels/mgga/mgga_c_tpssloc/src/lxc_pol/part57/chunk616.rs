//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 616/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk616<F: Float>(t1509: F, t1519: F, t252: F, t5584: F, t120: F, t5611: F, t225: F, t5559: F, t5632: F, t5561: F, t5849: F, t5851: F, t5915: F, t5385: F, t604: F, t111: F, t5449: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16758 = t1519 * t1509;
    let t16815 = t252 * t5584;
    let t16839 = t120 * t5584;
    let t16891 = t120 * t5611;
    let t17030 = t252 * t5611;
    let t17052 = t5559 * t225;
    let t17090 = t5632 * t225;
    let t17092 = t5561 * t225;
    let t17575 = t5849 * t225;
    let t17588 = t5851 * t225;
    let t18074 = t5915 * t225;
    let t19299 = t5385 * t604;
    let t19451 = t5449 * t111;
    (t16758, t16815, t16839, t16891, t17030, t17052, t17090, t17092, t17575, t17588, t18074, t19299, t19451)
}
