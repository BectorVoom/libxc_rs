//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 790/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk790<F: Float>(t235: F, t31984: F, t226: F, t31379: F, t31387: F, t31391: F, t31987: F, t31989: F, t31994: F, t808: F, t812: F, t8738: F, t858: F, t2053: F, t2718: F, t7106: F) -> (F, F, F, F) {
    let t31996 = t235 * t31984;
    let t31998 = -t31987 - 0.6579736267392905746e-1 * t31379 - t31989 - 0.3289868133696452873e-1 * t31387 + 0.3289868133696452873e-1 * t31391 + t808 * t8738 - t812 * t31994 + t226 * t31996;
    let t31999 = t858 * t31998;
    let t32002 = t2718 * t2053 * t7106;
    (t31996, t31998, t31999, t32002)
}
