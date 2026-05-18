//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 930/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk930<F: Float>(t1995: F, t22813: F, t116: F, t117: F, t67: F, t1999: F, t794: F, t61: F, t9222: F, t133: F, t6933: F, t6604: F, t6925: F) -> (F, F, F, F, F, F, F) {
    let t22814 = t22813 * t1995;
    let t22815 = t117 * t116;
    let t22816 = t67 * t22815;
    let t22817 = t22814 * t22816;
    let t22818 = t794 * t1999;
    let t22819 = t22817 * t22818;
    let t22820 = F::new(0.16821981705891829522e-4) * t22819;
    let t22822 = F::new(1.0) / t61 / t9222;
    let t22823 = t22822 * t1995;
    let t22824 = t22823 * t133;
    let t22825 = t22824 * t6933;
    let t22826 = F::new(0.52708876011794399171e-3) * t22825;
    let t22827 = t6925 * t6604;
    (t22816, t22817, t22820, t22822, t22824, t22826, t22827)
}
