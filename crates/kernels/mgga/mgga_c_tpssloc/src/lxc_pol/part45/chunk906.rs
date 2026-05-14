//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 906/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk906<F: Float>(t1992: F, t22635: F, t26989: F, t3888: F, t22716: F, t8612: F, t114178: F, t114188: F, t114193: F, t114209: F, t114217: F, t114220: F, t115540: F, t115542: F, t115547: F, t115551: F, t115554: F, t1375: F, t1385: F, t2015: F, t24138: F, t31555: F, t31601: F, t31641: F, t3758: F, t3882: F, t3887: F) -> (F,) {
    let t115558 = t1992 * t22635 * t26989 * t3888;
    let t115566 = t22716 * t8612;
    let t115567 = 0.63969658155208805863e-1 * t115566;
    let t115570 = -t114178 + 2.0 * t1375 * t3887 * t24138 * t2015 - t115540 - 0.16449340668482264365e-1 * t115542 + t114188 + 0.3289868133696452873e-1 * t115547 + t115551 + t114193 - 0.6579736267392905746e-1 * t115554 - 0.49348022005446793095e-1 * t115558 + 4.0 * t3758 * t31555 + 4.0 * t1375 * t3887 * t31641 * t1385 + t115567 - t114209 + t114217 - t114220 + 4.0 * t3882 * t31601;
    (t115570,)
}
