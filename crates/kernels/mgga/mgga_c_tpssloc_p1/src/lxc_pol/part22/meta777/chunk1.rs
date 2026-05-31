//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2656/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2656<F: Float>(t16288: F, t6417: F, t12385: F, t20497: F, t120: F, t12369: F, t12429: F, t1307: F, t1352: F, t1363: F, t16278: F, t16394: F, t19735: F, t19871: F, t19951: F, t19989: F, t20356: F, t20416: F, t20454: F, t3803: F, t3805: F, t40070: F, t5246: F, t5248: F, t53918: F, t53920: F, t54023: F, t54162: F, t6390: F, t6396: F, t6422: F, t74120: F, t820: F) -> F {
    let t74217 = t16288 * t6417;
    let t74228 = t12385 * t20497;
    let t74253 = F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t74217 - t16278 * t6422 / F::cast_from(1024.0_f64) + t54023 * t6390 / F::cast_from(512.0_f64) + F::cast_from(35.0_f64) / F::cast_from(128.0_f64) * t1363 * t40070 * t820 * t20356 * t1307 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t74228 - t53918 - t53920 + t12429 * t20454 / F::cast_from(256.0_f64) + t3803 * t3805 * t19871 * t19989 / F::cast_from(256.0_f64) + t54162 * t6396 / F::cast_from(128.0_f64) + t16394 * t19951 / F::cast_from(128.0_f64) - t5246 * t3805 * t74120 * t12369 / F::cast_from(128.0_f64) + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t5246 * t5248 * t19871 * t19735 + t3803 * t3805 * t120 * t20416 * t1352 / F::cast_from(768.0_f64);
    t74253
}
