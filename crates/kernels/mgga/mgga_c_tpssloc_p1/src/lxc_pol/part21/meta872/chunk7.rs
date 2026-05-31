//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3219/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3219<F: Float>(t16507: F, t1858: F, t3: F, t5364: F, t5381: F, t55368: F, t55374: F, t55376: F, t55378: F, t580: F, t66937: F, t66976: F, t66987: F, t66989: F, t66991: F) -> F {
    let t66993 = t3 * t580 * t66937 + F::cast_from(2.0_f64) * t16507 * t1858 + F::cast_from(4.0_f64) * t5364 * t5381 + F::cast_from(2.0_f64) * t55368 + F::cast_from(2.0_f64) * t55374 + F::cast_from(4.0_f64) * t55376 + F::cast_from(4.0_f64) * t55378 + F::cast_from(2.0_f64) * t66976 + F::cast_from(2.0_f64) * t66987 + F::cast_from(2.0_f64) * t66989 + F::cast_from(4.0_f64) * t66991;
    t66993
}
