//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2655/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2655<F: Float>(t12240: F, t12379: F, t12392: F, t12409: F, t16242: F, t16261: F, t16394: F, t16401: F, t3803: F, t40000: F, t40168: F, t40169: F, t5235: F, t5246: F, t5248: F, t5249: F, t54114: F, t54116: F, t54118: F, t54125: F, t54132: F, t54133: F, t54135: F) -> F {
    let t54137 = t5246 * t5248 * t16242 * t12240 / F::cast_from(512.0_f64) + t16401 * t16261 / F::cast_from(512.0_f64) + t5246 * t5248 * t5249 * t40000 / F::cast_from(1536.0_f64) + t16394 * t12409 / F::cast_from(256.0_f64) + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t54114 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t54116 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t54118 + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t3803 * t40168 * t5249 * t40169 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t54125 - t5235 * t12392 / F::cast_from(3072.0_f64) - t5235 * t12379 / F::cast_from(3072.0_f64) + t54132 - F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t54133 - F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t54135;
    t54137
}
