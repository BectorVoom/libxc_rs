//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2666/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2666<F: Float>(t12725: F, t12734: F, t12823: F, t12841: F, t1442: F, t1459: F, t15857: F, t19456: F, t20109: F, t20143: F, t2314: F, t2320: F, t3652: F, t4028: F, t4034: F, t4037: F, t4072: F, t510: F, t5107: F, t5118: F, t5361: F, t5457: F, t5460: F, t5494: F, t55946: F, t55962: F, t55967: F, t6287: F, t652: F) -> F {
    let t56034 = -F::cast_from(8.0_f64) * t4072 * t5107 * t652 - F::cast_from(8.0_f64) * t12725 * t4037 - F::cast_from(4.0_f64) * t12734 * t5494 - F::cast_from(4.0_f64) * t12823 * t5460 - F::cast_from(2.0_f64) * t12823 * t5494 - F::cast_from(4.0_f64) * t12841 * t4028 - F::cast_from(2.0_f64) * t1442 * t15857 - F::cast_from(4.0_f64) * t1459 * t55962 - F::cast_from(8.0_f64) * t19456 * t4037 - F::cast_from(8.0_f64) * t20109 * t4034 - F::cast_from(4.0_f64) * t20143 * t2314 - F::cast_from(4.0_f64) * t20143 * t4034 - F::cast_from(2.0_f64) * t2320 * t6287 - F::cast_from(2.0_f64) * t3652 * t5457 - F::cast_from(2.0_f64) * t510 * t55946 - F::cast_from(2.0_f64) * t510 * t55967 + F::cast_from(4.0_f64) * t5118 * t5361;
    t56034
}
