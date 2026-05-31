//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2270/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2270<F: Float>(t4021: F, t641: F, t72: F, t645: F, t7445: F, t1863: F, t22550: F, t7441: F, t12619: F, t71: F, t1860: F, t22490: F, t22493: F, t22512: F, t22549: F, t26009: F, t26021: F, t26024: F, t26025: F, t31683: F, t6486: F, t6490: F, t6505: F, t7428: F, t7442: F, t7446: F, t9239: F) -> F {
    let t90232 = t72 * t641 * t4021;
    let t90247 = t7445 * t645;
    let t90248 = t1863 * t90247;
    let t90251 = t7441 * t22550;
    let t90257 = t71 * t12619;
    let t90265 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t6490 * t90232 - t22493 * t7446 / F::cast_from(6.0_f64) - t6486 * t26021 / F::cast_from(3.0_f64) - t6486 * t26025 / F::cast_from(3.0_f64) - t1860 * t22512 * t7445 / F::cast_from(6.0_f64) + F::cast_from(20.0_f64) * t9239 * t31683 * t26009 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t22549 * t90248 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t22549 * t90251 - t1860 * t6505 * t26024 / F::cast_from(3.0_f64) - t1860 * t1863 * t90257 / F::cast_from(6.0_f64) - t7428 * t22490 / F::cast_from(6.0_f64) - t22493 * t7442 / F::cast_from(6.0_f64);
    t90265
}
