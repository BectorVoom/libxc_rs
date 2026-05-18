//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 382/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk382<F: Float>(t815: F, t835: F, t812: F, t242: F, t67: F, t845: F, t246: F, t152: F, t32: F, t181: F, t204: F, t686: F) -> (F, F, F, F, F) {
    let t2638 = t815 * t835;
    let t2639 = t812 * t2638;
    let t2642 = t815 * t242;
    let t2643 = t812 * t2642;
    let t2644 = t845 * t67;
    let t2645 = t2644 * t246;
    let t2658 = t32 * t152;
    let t2663 = t686 * t204 * t181;
    (t2639, t2643, t2645, t2658, t2663)
}
