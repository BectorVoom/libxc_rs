//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 998/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk998<F: Float>(t23035: F, t28298: F, t31366: F, t121401: F, t6552: F, t7479: F, t121399: F, t126286: F, t126290: F, t126291: F, t126349: F, t126352: F, t126353: F, t126358: F, t17090: F, t2053: F, t25188: F, t2718: F, t28431: F, t29080: F, t33443: F, t33452: F, t4147: F, t4268: F, t6627: F, t7830: F, t855: F, t8553: F) -> F {
    let t127847 = t23035 * t31366 * t28298;
    let t127852 = t6552 * t121401 * t7479;
    let t127858 = -t126286 + t126290 + t126291 + F::new(0.82246703342411321824e-2) * t121399 + F::new(4.0) * t25188 * t7830 + F::new(2.0) * t855 * t2718 * t2053 * t28431 + F::new(4.0) * t4147 * t33443 + F::new(4.0) * t4268 * t33443 + F::new(0.49348022005446793095e-1) * t127847 - t126349 - t126352 - t126353 + F::new(4.0) * t4268 * t33452 + t126358 - F::new(0.3289868133696452873e-1) * t127852 + F::new(2.0) * t17090 * t8553 + F::new(4.0) * t6627 * t29080;
    t127858
}
