//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1093/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1093<F: Float>(t32822: F, t6646: F, t1888: F, t1894: F, t7510: F, t214: F, t1880: F, t1510: F, t30694: F, t1484: F, t59: F, t6591: F, t6612: F, t6605: F, t1499: F, t8342: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32823 = t6646 * t32822;
    let t32825 = 0.16449340668482264365e-1 * t1888 * t32823;
    let t32826 = t1894 * t7510;
    let t32827 = t214 * t32826;
    let t32829 = 0.16449340668482264365e-1 * t1880 * t32827;
    let t32831 = t30694 * t1510;
    let t32834 = t1894 * t59 * t1484;
    let t32835 = t6591 * t32834;
    let t32837 = t6612 * t1510;
    let t32838 = t6605 * t32837;
    let t32840 = t1499 * t8342;
    (t32823, t32825, t32826, t32827, t32829, t32831, t32834, t32835, t32837, t32838, t32840)
}
