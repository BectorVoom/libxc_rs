//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 868/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk868<F: Float>(t1912: F, t2054: F, t259: F, t6627: F, t7087: F, t8334: F, t8338: F, t8539: F, t8544: F, t8549: F, t855: F, t8553: F, t8563: F) -> F {
    let t8565 = t8334 - t8338 + F::cast_from(0.82246703342411321825e-2_f64) * t8539 + t8544 * t259 - t7087 * t1912 - F::cast_from(0.82246703342411321825e-2_f64) * t8549 - t6627 * t2054 + F::new(2.0) * t855 * t8553 - t855 * t8563;
    t8565
}
