//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1477/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1477<F: Float>(t16558: F, t31: F, t65: F, t5399: F, t628: F, t1426: F, t3961: F, t3967: F, t1410: F, t3997: F, t1434: F, t19322: F, t19323: F, t19326: F, t19331: F, t3962: F, t5393: F, t5400: F, t5403: F, t642: F, t80: F) -> (F, F) {
    let t19334 = t31 * t16558;
    let t19335 = t19334 * t65;
    let t19338 = t5399 * t628;
    let t19343 = t3961 * t1426;
    let t19346 = t3967 * t1426;
    let t19349 = t1410 * t3997;
    let t19356 = -t19322 * t19323 / F::new(6.0) - t19326 * t80 / F::new(12.0) - t5393 * t642 / F::new(12.0) - t19331 * t80 / F::new(12.0) - t19335 * t80 / F::new(12.0) - t19338 * t80 / F::new(12.0) - t5400 * t642 / F::new(12.0) - t19343 * t80 / F::new(6.0) - t19346 * t80 / F::new(6.0) - t19349 * t80 / F::new(6.0) - t5403 * t642 / F::new(6.0) - t3962 * t1434 / F::new(6.0);
    (t19334, t19356)
}
