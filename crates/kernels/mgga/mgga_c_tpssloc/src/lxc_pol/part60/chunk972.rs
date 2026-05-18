//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 972/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk972<F: Float>(t118903: F, t1880: F, t28431: F, t6553: F, t6571: F, t118678: F, t1888: F, t232: F, t6646: F, t98541: F, t22996: F, t2632: F) -> (F, F, F, F, F) {
    let t126423 = F::new(0.16449340668482264365e-1) * t118903;
    let t126427 = F::new(0.16449340668482264365e-1) * t1880 * t6553 * t6571 * t28431;
    let t126433 = F::new(0.76763589786250567036e-1) * t118678;
    let t126437 = F::new(0.16449340668482264365e-1) * t1888 * t6646 * t98541 * t232;
    let t126441 = F::new(0.3289868133696452873e-1) * t1888 * t22996 * t98541 * t2632;
    (t126423, t126427, t126433, t126437, t126441)
}
