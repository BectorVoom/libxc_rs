//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2655/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2655<F: Float>(t12240: F, t12379: F, t12392: F, t12409: F, t16242: F, t16261: F, t16394: F, t16401: F, t3803: F, t40000: F, t40168: F, t40169: F, t5235: F, t5246: F, t5248: F, t5249: F, t54114: F, t54116: F, t54118: F, t54125: F, t54132: F, t54133: F, t54135: F) -> F {
    let t54137 = t5246 * t5248 * t16242 * t12240 / F::new(512.0) + t16401 * t16261 / F::new(512.0) + t5246 * t5248 * t5249 * t40000 / F::new(1536.0) + t16394 * t12409 / F::new(256.0) + F::new(7.0) / F::new(768.0) * t54114 - F::new(7.0) / F::new(384.0) * t54116 - F::new(7.0) / F::new(384.0) * t54118 + F::new(5.0) / F::new(128.0) * t3803 * t40168 * t5249 * t40169 + F::new(7.0) / F::new(768.0) * t54125 - t5235 * t12392 / F::new(3072.0) - t5235 * t12379 / F::new(3072.0) + t54132 - F::new(35.0) / F::new(192.0) * t54133 - F::new(35.0) / F::new(192.0) * t54135;
    t54137
}
