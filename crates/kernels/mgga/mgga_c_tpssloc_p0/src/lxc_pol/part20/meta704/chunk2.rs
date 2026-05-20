//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2676/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2676<F: Float>(t12012: F, t12147: F, t12164: F, t1347: F, t1348: F, t16176: F, t16186: F, t16196: F, t16199: F, t1819: F, t1821: F, t225: F, t3839: F, t3847: F, t5272: F, t5278: F, t5279: F, t5283: F, t53856: F, t54311: F, t54377: F, t54391: F, t54415: F, t54426: F, t54440: F, t54454: F, t54479: F, t54525: F, t546: F, t548: F, t550: F) -> F {
    let t54527 = (-F::new(12.0) * t5278 * t5279 * t12012 + F::new(9.0) * t5272 * t3847 + F::new(3.0) * t546 * t1347 * t53856 + F::new(3.0) * t1819 * t12164 + F::new(3.0) * t12147 * t1821 - (t54311 + t54377 + t54391 + t54415 + t54426 + t54440 + t54454 + t54479) * t225 * t548 + F::new(9.0) * t16176 * t1348 + F::new(9.0) * t3839 * t5283 - F::new(72.0) * t16186 * t16196 - F::new(36.0) * t16186 * t16199 + t54525) * t550;
    t54527
}
