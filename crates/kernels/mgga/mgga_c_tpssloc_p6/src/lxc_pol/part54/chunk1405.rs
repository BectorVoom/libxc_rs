//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1405/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1405<F: Float>(t100688: F, t101840: F, t119746: F, t119780: F, t121264: F, t1877: F, t24191: F, t2522: F, t25901: F, t25930: F, t26744: F, t26756: F, t30974: F, t31434: F, t31441: F, t31448: F, t31502: F, t33483: F, t33537: F, t7114: F, t8566: F, t89849: F, t89992: F, t92271: F) -> F {
    let t121982 = t26756 * t89849 * t33483 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8566 * t25901 + t101840 * t31502 - t1877 * t7114 * t119746 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t89992 * t31441 - t1877 * t26744 * t30974 / F::cast_from(2.0_f64) + t92271 * t33537 + t121264 + t26756 * t100688 * t31448 - t1877 * t31434 * t25930 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t119780;
    t121982
}
