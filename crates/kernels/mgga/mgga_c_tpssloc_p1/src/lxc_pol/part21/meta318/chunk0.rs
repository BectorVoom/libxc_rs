//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1686/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1686<F: Float>(t225: F, t3591: F, t3482: F, t3639: F, t500: F, t3696: F, t588: F, t592: F, t1285: F, t2223: F, t1287: F, t1291: F, t9874: F) -> (F, F, F, F, F, F, F, F) {
    let t11925 = t3591 * t225;
    let t11928 = t3482 * t225;
    let t11947 = F::cast_from(1.0_f64) / t3639 / t500;
    let t11975 = t588 * t3696;
    let t11977 = t592 * t3696;
    let t11979 = t2223 * t1285;
    let t11981 = t2223 * t1287;
    let t11984 = F::cast_from(0.56968947174242584612e-3_f64) * t1291 * t9874;
    (t11925, t11928, t11947, t11975, t11977, t11979, t11981, t11984)
}
