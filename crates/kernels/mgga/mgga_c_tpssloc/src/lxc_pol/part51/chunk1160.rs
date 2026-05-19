//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1160/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1160<F: Float>(t31381: F, t6562: F, t2047: F, t232: F, t828: F, t6646: F, t1888: F, t1894: F, t7084: F, t214: F, t1880: F, t814: F, t8543: F) -> (F, F, F, F, F, F, F, F) {
    let t31382 = t6562 * t31381;
    let t31383 = F::cast_from(0.41123351671205660912e-2_f64) * t31382;
    let t31385 = t2047 * t828 * t232;
    let t31386 = t6646 * t31385;
    let t31387 = t1888 * t31386;
    let t31389 = t1894 * t7084;
    let t31390 = t214 * t31389;
    let t31391 = t1880 * t31390;
    let t31394 = t814 * t8543;
    (t31383, t31385, t31386, t31387, t31389, t31390, t31391, t31394)
}
