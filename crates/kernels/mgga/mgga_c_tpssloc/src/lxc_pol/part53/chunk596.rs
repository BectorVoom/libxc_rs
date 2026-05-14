//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 596/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk596<F: Float>(t1484: F, t28: F, t1530: F, t1458: F, t88: F, t1778: F, t191: F, t192: F) -> (F, F, F, F) {
    let t7649 = t28 * t1484;
    let t7656 = t28 * t1530;
    let t7676 = t88 * t1458;
    let t7684 = t1778 * t191;
    let t7685 = t7684 * t192;
    (t7649, t7656, t7676, t7685)
}
