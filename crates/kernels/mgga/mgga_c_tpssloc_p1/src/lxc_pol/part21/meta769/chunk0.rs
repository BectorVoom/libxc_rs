//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2665/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2665<F: Float>(t12550: F, t12557: F, t12725: F, t12734: F, t12813: F, t1459: F, t1774: F, t19451: F, t19456: F, t20109: F, t2314: F, t2323: F, t26114: F, t4028: F, t4073: F, t4077: F, t45632: F, t5460: F, t5494: F, t55934: F, t55969: F, t574: F, t652: F, t7458: F, t9348: F) -> F {
    let t55998 = -F::cast_from(4.0_f64) * t12813 * t1774 * t652 - F::cast_from(8.0_f64) * t12550 * t4028 - F::cast_from(8.0_f64) * t12550 * t7458 - F::cast_from(4.0_f64) * t12557 * t4028 - F::cast_from(8.0_f64) * t12725 * t4073 - F::cast_from(8.0_f64) * t12725 * t4077 - F::cast_from(8.0_f64) * t12734 * t5460 - F::cast_from(4.0_f64) * t1459 * t45632 - F::cast_from(8.0_f64) * t1459 * t55934 - F::cast_from(4.0_f64) * t19451 * t2323 - F::cast_from(8.0_f64) * t19456 * t4073 - F::cast_from(8.0_f64) * t20109 * t2314 - F::cast_from(8.0_f64) * t26114 * t4073 - F::cast_from(4.0_f64) * t5460 * t9348 - F::cast_from(2.0_f64) * t5494 * t9348 + t55969 * t574;
    t55998
}
