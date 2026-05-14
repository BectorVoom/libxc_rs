//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 510/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk510<F: Float>(t252: F, t828: F, t232: F, t6646: F, t1888: F, t1894: F, t852: F, t214: F, t1880: F, t1902: F, t814: F, t829: F, t235: F, t6624: F, t1909: F, t226: F, t6636: F, t6641: F, t6645: F, t808: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6647 = t252 * t828;
    let t6648 = t6647 * t232;
    let t6649 = t6646 * t6648;
    let t6650 = t1888 * t6649;
    let t6652 = t1894 * t852;
    let t6653 = t214 * t6652;
    let t6654 = t1880 * t6653;
    let t6657 = t814 * t1902;
    let t6658 = t6657 * t829;
    let t6660 = t235 * t6624;
    let t6662 = -t6636 - 0.16449340668482264365e-1 * t6641 - t6645 - 0.82246703342411321825e-2 * t6650 + 0.82246703342411321825e-2 * t6654 + t808 * t1909 - t812 * t6658 + t226 * t6660;
    (t6648, t6649, t6650, t6652, t6653, t6654, t6657, t6658, t6660, t6662)
}
