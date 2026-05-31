//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2664/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2664<F: Float>(t1268: F, t12725: F, t12734: F, t12739: F, t12813: F, t1458: F, t19451: F, t19456: F, t19534: F, t2314: F, t2363: F, t26114: F, t4028: F, t4072: F, t45632: F, t5113: F, t5493: F, t55410: F, t55568: F, t55927: F, t55934: F, t55943: F, t55946: F, t55962: F, t55967: F, t671: F, t7676: F, t88: F, t9348: F) -> F {
    let t55969 = F::cast_from(2.0_f64) * t1268 * t55568 + F::cast_from(8.0_f64) * t12725 * t4072 + F::cast_from(4.0_f64) * t12734 * t5493 + F::cast_from(2.0_f64) * t12739 * t5493 + F::cast_from(4.0_f64) * t12813 * t4028 + F::cast_from(4.0_f64) * t12813 * t7676 + F::cast_from(4.0_f64) * t1458 * t45632 + F::cast_from(8.0_f64) * t1458 * t55934 + F::cast_from(4.0_f64) * t1458 * t55962 + F::cast_from(2.0_f64) * t19451 * t2363 + F::cast_from(8.0_f64) * t19456 * t4072 + F::cast_from(4.0_f64) * t19534 * t2314 + F::cast_from(4.0_f64) * t19534 * t5113 + F::cast_from(8.0_f64) * t26114 * t4072 + F::cast_from(2.0_f64) * t5493 * t9348 + F::cast_from(4.0_f64) * t55410 * t88 + F::cast_from(4.0_f64) * t55943 * t671 + t55927 + F::cast_from(2.0_f64) * t55946 + F::cast_from(2.0_f64) * t55967;
    t55969
}
