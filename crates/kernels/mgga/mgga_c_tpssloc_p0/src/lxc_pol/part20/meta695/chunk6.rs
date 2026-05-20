//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2653/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2653<F: Float>(t12283: F, t16405: F, t40167: F, t820: F, t1799: F, t3791: F, t40138: F, t5259: F, t16248: F, t5293: F, t16275: F, t120: F, t12178: F, t12420: F, t12429: F, t1352: F, t16018: F, t16224: F, t16225: F, t16227: F, t16364: F, t16370: F, t16387: F, t16391: F, t16401: F, t3793: F, t3803: F, t3805: F, t3807: F, t5246: F, t5248: F, t5249: F) -> (F, F) {
    let t54059 = t12283 * t16405;
    let t54063 = t40167 * t820;
    let t54068 = t1799 * t3791;
    let t54086 = t40138 * t5259;
    let t54088 = t12283 * t16248;
    let t54090 = t40138 * t5293;
    let t54092 = t12283 * t16275;
    let t54100 = F::new(35.0) / F::new(384.0) * t54059 - F::new(5.0) / F::new(128.0) * t12429 * t16227 + F::new(15.0) / F::new(128.0) * t3803 * t54063 * t16225 * t12420 - F::new(5.0) / F::new(256.0) * t3803 * t16224 * t54068 * t3807 + t12429 * t16370 / F::new(256.0) + t3803 * t3805 * t120 * t16018 * t1352 / F::new(256.0) - t16401 * t16391 / F::new(128.0) - t5246 * t3805 * t16364 * t3793 / F::new(128.0) - F::new(7.0) / F::new(192.0) * t54086 - F::new(7.0) / F::new(384.0) * t54088 + F::new(7.0) / F::new(768.0) * t54090 + F::new(7.0) / F::new(1536.0) * t54092 + F::new(3.0) / F::new(512.0) * t16401 * t16387 - t3803 * t5248 * t5249 * t12178 / F::new(3072.0);
    (t54068, t54100)
}
