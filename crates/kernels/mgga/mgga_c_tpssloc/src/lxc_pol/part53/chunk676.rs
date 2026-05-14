//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 676/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk676<F: Float>(t116: F, t117: F, t67: F, t22814: F, t1999: F, t794: F, t61: F, t9222: F, t1995: F, t133: F, t6933: F, t6604: F, t6925: F, t242: F, t6943: F, t1336: F) -> (F, F, F, F, F, F, F, F) {
    let t22815 = t117 * t116;
    let t22816 = t67 * t22815;
    let t22817 = t22814 * t22816;
    let t22818 = t794 * t1999;
    let t22819 = t22817 * t22818;
    let t22822 = 1.0 / t61 / t9222;
    let t22823 = t22822 * t1995;
    let t22824 = t22823 * t133;
    let t22825 = t22824 * t6933;
    let t22827 = t6925 * t6604;
    let t22832 = t6943 * t242;
    let t22833 = t1336 * t22832;
    (t22816, t22817, t22819, t22822, t22824, t22825, t22827, t22833)
}
