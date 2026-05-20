//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2698/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2698<F: Float>(t1338: F, t16413: F, t12168: F, t12181: F, t12238: F, t1332: F, t1336: F, t1352: F, t1380: F, t1381: F, t16052: F, t16055: F, t16060: F, t16206: F, t16414: F, t1825: F, t1840: F, t3901: F, t3907: F, t40479: F, t5234: F, t5348: F, t53909: F, t54527: F) -> F {
    let t55039 = t1338 * t16413;
    let t55059 = -t12168 * t1336 * t5348 - F::new(3.0) * t1336 * t1352 * t55039 - t1336 * t1380 * t54527 - F::new(3.0) * t1336 * t16206 * t3901 - t1336 * t1825 * t40479 - F::new(3.0) * t12181 * t5234 + t12238 * t1840 + F::new(3.0) * t1332 * t16414 - F::new(3.0) * t1381 * t53909 + F::new(18.0) * t16052 * t16055 - F::new(3.0) * t16060 * t3907;
    t55059
}
