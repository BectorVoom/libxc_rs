//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2334/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2334<F: Float>(t12369: F, t19871: F, t3805: F, t12346: F, t12366: F, t12429: F, t1363: F, t16233: F, t16394: F, t16400: F, t19940: F, t19942: F, t19945: F, t19951: F, t19958: F, t19962: F, t19966: F, t19972: F, t19976: F, t19981: F, t19986: F, t19991: F, t19996: F, t20000: F, t3803: F, t5246: F, t5259: F, t6396: F) -> (F, F) {
    let t20004 = t3805 * t19871 * t12369;
    let t20007 = -F::cast_from(35.0_f64) / F::cast_from(1152.0_f64) * t19940 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t19942 + t5246 * t19945 / F::cast_from(768.0_f64) + t12429 * t6396 / F::cast_from(384.0_f64) + t3803 * t19951 / F::cast_from(384.0_f64) - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t12346 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t12366 + t3803 * t19958 / F::cast_from(768.0_f64) - t3803 * t19962 / F::cast_from(3072.0_f64) + t5246 * t19966 / F::cast_from(1536.0_f64) + t16394 * t5259 / F::cast_from(384.0_f64) - t3803 * t19972 / F::cast_from(1536.0_f64) - t3803 * t19976 / F::cast_from(3072.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3803 * t19981 + t3803 * t19986 / F::cast_from(768.0_f64) + t3803 * t19991 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1363 * t19996 - t16400 - t16233 * t20000 / F::cast_from(512.0_f64) - t5246 * t20004 / F::cast_from(384.0_f64);
    (t20004, t20007)
}
