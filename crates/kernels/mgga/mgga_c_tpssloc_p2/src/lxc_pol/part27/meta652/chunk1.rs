//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2275/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2275<F: Float>(t1873: F, t90375: F, t22479: F, t4028: F, t1458: F, t2363: F, t24999: F, t83935: F, t90351: F, t90352: F, t90355: F, t90361: F, t90363: F, t90365: F, t90367: F, t90369: F, t90372: F, t90374: F) -> F {
    let t90377 = F::new(2.0) * t90375 * t1873;
    let t90379 = F::new(2.0) * t4028 * t22479;
    let t90380 = F::new(2.0) * t1458 * t83935 + F::new(2.0) * t2363 * t24999 + t90351 + F::new(2.0) * t90352 + t90355 + t90361 + t90363 + t90365 + t90367 + t90369 + t90372 + t90374 + t90377 + t90379;
    t90380
}
