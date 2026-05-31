//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1155/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1155<F: Float>(t1314: F, t9569: F, t1329: F, t12189: F, t3770: F, t12279: F, t12303: F, t12368: F, t12371: F, t12419: F, t1352: F, t16224: F, t16401: F, t3803: F, t3805: F, t3806: F, t3809: F, t39971: F, t39973: F, t39975: F, t39978: F, t39983: F, t39989: F, t39993: F, t40000: F, t5246: F, t5248: F) -> (F, F) {
    let t40005 = t9569 * t1314;
    let t40006 = t40005 * t1329;
    let t40008 = t12189 * t3770;
    let t40010 = -F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t39971 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t39973 + t39975 * t3809 / F::cast_from(64.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t5246 * t12419 * t12368 * t39978 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t39983 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t3803 * t16224 * t1352 * t12303 - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t39989 - t16401 * t12371 / F::cast_from(32.0_f64) - t5246 * t3805 * t12368 * t39993 / F::cast_from(64.0_f64) + t16401 * t12279 / F::cast_from(128.0_f64) + t5246 * t5248 * t3806 * t40000 / F::cast_from(384.0_f64) + F::cast_from(455.0_f64) / F::cast_from(162.0_f64) * t40006 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t40008;
    (t40005, t40010)
}
