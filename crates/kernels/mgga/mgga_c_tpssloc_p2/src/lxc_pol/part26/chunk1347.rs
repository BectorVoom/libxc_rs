//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1347/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1347<F: Float>(t7291: F, t85660: F, t24564: F, t24574: F, t1090: F, t11504: F, t1186: F, t1251: F, t2123: F, t2250: F, t24589: F, t24590: F, t24601: F, t24602: F, t24611: F, t24880: F, t24887: F, t27549: F, t3631: F, t7283: F, t7287: F, t85628: F, t85640: F, t85642: F, t85643: F, t85648: F, t85652: F) -> F {
    let t85661 = t85660 * t7291;
    let t85669 = t24574 * t24564;
    let t85673 = F::cast_from(0.82246703342411321826e-2_f64) * t24589 * t24601 * t85628 * t1090 + F::cast_from(0.82246703342411321826e-2_f64) * t24589 * t24601 * t24602 * t2250 * t1251 + F::cast_from(0.54831135561607547883e-2_f64) * t85640 - F::cast_from(0.10966227112321509577e-1_f64) * t27549 * t24601 * t85642 * t85643 + F::cast_from(0.82246703342411321826e-2_f64) * t24589 * t85648 * t7287 + F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t24601 * t85652 * t85643 + F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t24590 * t24887 + F::cast_from(0.54831135561607547884e-2_f64) * t85661 - F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t1186 * t24611 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t11504 * t2123 - F::cast_from(0.82246703342411321826e-2_f64) * t85669 - F::cast_from(3.0_f64) * t24880 * t3631;
    t85673
}
