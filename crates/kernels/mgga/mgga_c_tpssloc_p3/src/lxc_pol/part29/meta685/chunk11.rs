//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2349/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2349<F: Float>(t90372: F, t90374: F, t90377: F, t90379: F, t90383: F, t90385: F, t90387: F, t90399: F, t90404: F, t90406: F, t90408: F, t90410: F, t94265: F, t96214: F) -> F {
    let t96271 = t90372 + t90374 + t90377 + t90379 + t90383 + t90385 + t90387 + t90399 + t90404 + t90406 + t90408 + t90410 + t96214 + F::new(2.0) * t94265;
    t96271
}
