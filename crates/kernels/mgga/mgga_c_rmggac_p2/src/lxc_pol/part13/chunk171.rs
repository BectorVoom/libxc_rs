//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 171/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk171<F: Float>(t326: F, t559: F, t305: F, t344: F, t349: F, t551: F, t558: F) -> (F, F) {
    let t560 = t326 * t559;
    let t570 = F::cast_from(0.19957069503106347607e-1_f64) * t305 * t551 - F::cast_from(0.19957069503106347607e-1_f64) * t326 * t558 + F::cast_from(0.26552308210121162678e-3_f64) * t344 * t551 - F::cast_from(0.26552308210121162678e-3_f64) * t349 * t558;
    (t560, t570)
}
