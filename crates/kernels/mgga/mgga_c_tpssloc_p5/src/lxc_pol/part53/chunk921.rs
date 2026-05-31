//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 921/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk921<F: Float>(t33973: F, t858: F, t1527: F, t8740: F, t2718: F, t1528: F, t2054: F, t26700: F, t26713: F, t31964: F, t32023: F, t32027: F, t33449: F, t33459: F, t4147: F, t4268: F, t855: F, t8734: F, t8741: F) -> (F, F, F) {
    let t33974 = t858 * t33973;
    let t33981 = t8740 * t1527;
    let t33982 = t2718 * t33981;
    let t33989 = -F::cast_from(2.0_f64) * t26713 * t2054 + F::cast_from(2.0_f64) * t4268 * t8734 - t855 * t33974 + F::cast_from(2.0_f64) * t4147 * t8734 + t32023 + F::cast_from(0.6579736267392905746e-1_f64) * t33449 + F::cast_from(0.6579736267392905746e-1_f64) * t33459 - t31964 * t1528 + F::cast_from(2.0_f64) * t855 * t33982 - F::cast_from(2.0_f64) * t26700 * t2054 - t4147 * t8741 - t4268 * t8741 + t32027;
    (t33974, t33982, t33989)
}
