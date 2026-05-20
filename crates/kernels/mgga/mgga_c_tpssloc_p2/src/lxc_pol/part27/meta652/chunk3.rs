//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2277/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2277<F: Float>(t12739: F, t7467: F, t26135: F, t5113: F, t12813: F, t1458: F, t22461: F, t26103: F, t4072: F, t6517: F, t671: F, t90041: F, t90044: F, t90383: F, t90385: F, t90387: F, t90399: F, t90400: F, t90404: F, t90406: F) -> F {
    let t90408 = F::new(2.0) * t12739 * t7467;
    let t90410 = F::new(4.0) * t5113 * t26135;
    let t90411 = F::new(2.0) * t12813 * t6517 + F::new(4.0) * t1458 * t90041 + F::new(2.0) * t1458 * t90044 + F::new(4.0) * t22461 * t4072 + F::new(4.0) * t26103 * t4072 + F::new(4.0) * t671 * t90400 + t90383 + t90385 + t90387 + t90399 + t90404 + t90406 + t90408 + t90410;
    t90411
}
