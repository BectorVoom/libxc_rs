//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2690/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2690<F: Float>(t12283: F, t16244: F, t1307: F, t3791: F, t12279: F, t12419: F, t12420: F, t12422: F, t12426: F, t12429: F, t16233: F, t16242: F, t16305: F, t16366: F, t16394: F, t19876: F, t3793: F, t3803: F, t39975: F, t40329: F, t5246: F, t5248: F, t5249: F, t5259: F, t5303: F, t54014: F, t54739: F, t54744: F, t54745: F, t54750: F, t554: F, t559: F) -> F {
    let t54764 = t12283 * t16244;
    let t54770 = t3791 * t1307;
    let t54776 = F::new(3.0) / F::new(512.0) * t5246 * t5248 * t16242 * t3793 + t54739 * t554 * t559 / F::new(3072.0) + t54744 * t5248 * t5249 * t54745 / F::new(128.0) + F::new(7.0) / F::new(192.0) * t54750 - F::new(5.0) / F::new(256.0) * t3803 * t12419 * t16242 * t12420 + t39975 * t5259 / F::new(256.0) + t19876 * t12279 / F::new(512.0) + t16394 * t12426 / F::new(256.0) - F::new(5.0) / F::new(256.0) * t16394 * t12422 - F::new(7.0) / F::new(192.0) * t54764 + t39975 * t5303 / F::new(256.0) + t12429 * t16366 / F::new(128.0) + F::new(3.0) / F::new(128.0) * t16233 * t16305 * t54014 * t54770 - F::new(7.0) / F::new(4608.0) * t40329;
    t54776
}
