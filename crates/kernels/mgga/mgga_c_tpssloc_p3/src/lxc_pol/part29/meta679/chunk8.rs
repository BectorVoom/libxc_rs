//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2284/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2284<F: Float>(t24574: F, t27412: F, t5052: F, t7299: F, t14972: F, t15359: F, t15790: F, t1716: F, t2123: F, t24596: F, t24601: F, t24617: F, t27381: F, t27396: F, t27406: F, t27549: F, t27820: F, t3243: F, t3593: F, t4930: F, t7283: F, t7295: F, t7302: F, t7351: F, t7392: F, t85787: F, t85789: F, t86452: F) -> F {
    let t94535 = F::cast_from(0.10966227112321509577e-1_f64) * t24574 * t27412;
    let t94558 = t7299 * t5052;
    let t94564 = -F::new(2.0) * t14972 * t7392 + t94535 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t15359 * t2123 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t4930 * t7295 + F::new(4.0) * t7351 * t15790 - F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t24617 - F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t24601 * t27381 * t3243 - F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t27820 * t24596 + F::cast_from(0.18277045187202515961e-2_f64) * t85787 + F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t1716 * t86452 - F::cast_from(0.18277045187202515961e-2_f64) * t85789 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t94558 * t7302 + F::new(4.0) * t3593 * t27396;
    t94564
}
