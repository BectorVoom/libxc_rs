//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1079/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1079<F: Float>(t1336: F, t22707: F, t24099: F, t26379: F, t26381: F, t26386: F, t26390: F, t26398: F, t26412: F, t26416: F, t26419: F, t26424: F, t26427: F, t27075: F, t27078: F, t27082: F, t27086: F, t27088: F, t3777: F, t5234: F, t5334: F, t5344: F, t7209: F, t7932: F) -> F {
    let t27095 = F::new(0.3289868133696452873e-1) * t26379 + F::new(0.76763589786250567037e-1) * t26381 + F::new(2.0) * t5334 * t27075 - t24099 - t5344 * t27078 - F::new(0.3289868133696452873e-1) * t26386 - F::new(0.3289868133696452873e-1) * t26390 + t27082 - F::new(0.3289868133696452873e-1) * t26398 - t5234 * t7209 - t3777 * t7932 - t1336 * t27086 + t27088 + F::new(0.82246703342411321825e-2) * t22707 - F::new(0.16449340668482264365e-1) * t26412 + F::new(0.3289868133696452873e-1) * t26416 - F::new(0.16449340668482264365e-1) * t26419 + F::new(0.3289868133696452873e-1) * t26424 + F::new(0.82246703342411321825e-2) * t26427;
    t27095
}
