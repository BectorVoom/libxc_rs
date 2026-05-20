//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1986/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1986<F: Float>(t87581: F, t87583: F, t2047: F, t4233: F, t87601: F, t87603: F, t13176: F, t24270: F, t2617: F, t26608: F, t26656: F, t4166: F, t4281: F, t4291: F, t7102: F, t81656: F, t81670: F, t81691: F, t829: F, t84995: F, t87575: F, t87578: F, t87589: F, t87609: F, t9632: F) -> (F, F) {
    let t92738 = F::cast_from(0.16449340668482264365e-1_f64) * t87581;
    let t92739 = F::cast_from(0.15352717957250113407e0_f64) * t87583;
    let t92745 = t2047 * t4233;
    let t92749 = F::cast_from(0.16449340668482264365e-1_f64) * t87601;
    let t92754 = F::cast_from(0.15352717957250113407e0_f64) * t87603;
    let t92759 = F::cast_from(0.3289868133696452873e-1_f64) * t81656 - F::cast_from(0.3289868133696452873e-1_f64) * t87575 - F::cast_from(0.16449340668482264365e-1_f64) * t87578 + t92738 - t92739 + F::cast_from(0.16449340668482264365e-1_f64) * t81670 - F::cast_from(0.6579736267392905746e-1_f64) * t87589 + F::new(2.0) * t4281 * t26656 * t9632 - F::new(2.0) * t4291 * t92745 * t829 + t92749 - F::new(2.0) * t13176 * t7102 - F::new(2.0) * t4166 * t24270 + t92754 - F::new(2.0) * t2617 * t26608 - t84995 + F::cast_from(0.82246703342411321825e-2_f64) * t81691 + F::cast_from(0.3289868133696452873e-1_f64) * t87609;
    (t92745, t92759)
}
