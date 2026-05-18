//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 808/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk808<F: Float>(t1912: F, t259: F, t6627: F, t8334: F, t8338: F, t8348: F, t8353: F, t8363: F, t855: F) -> F {
    let t8365 = -F::new(2.0) * t1912 * t6627 + t259 * t8348 + F::new(2.0) * t8353 * t855 - t8363 * t855 + t8334 - t8338;
    t8365
}
