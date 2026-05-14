//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 907/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk907<F: Float>(t1985: F, t26193: F, t33296: F, t127430: F, t22633: F, t22635: F, t31558: F, t122124: F, t1799: F, t1992: F, t26989: F, t6439: F, t102917: F, t114225: F, t114264: F, t122331: F, t127354: F, t127355: F, t127422: F, t127423: F, t127427: F, t20029: F, t2016: F, t26224: F, t26477: F, t27009: F, t28110: F, t7750: F, t7937: F, t8637: F) -> (F,) {
    let t128797 = t1985 * t26193 * t33296;
    let t128805 = t22633 * t22635 * t31558 * t127430;
    let t128809 = t22633 * t22635 * t122124 * t1799;
    let t128816 = t1992 * t22635 * t26989 * t6439;
    let t128818 = t114225 - 2.0 * t20029 * t8637 - 2.0 * t27009 * t7750 + t127354 + t127355 + 0.16449340668482264365e-1 * t122331 - 0.16449340668482264365e-1 * t128797 - 2.0 * t26477 * t7937 - 2.0 * t102917 * t2016 - 0.6579736267392905746e-1 * t128805 + t114264 - t127422 + 0.3289868133696452873e-1 * t128809 + t127423 - 6.0 * t26224 * t26989 * t28110 - 0.49348022005446793095e-1 * t128816 + t127427;
    (t128818,)
}
