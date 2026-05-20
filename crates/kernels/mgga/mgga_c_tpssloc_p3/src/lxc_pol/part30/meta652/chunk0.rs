//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2066/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2066<F: Float>(t7611: F, t82716: F, t25550: F, t82822: F, t23384: F, t25476: F, t25467: F, t25459: F, t7604: F, t82632: F, t25723: F, t88810: F) -> (F, F, F, F, F, F, F) {
    let t89310 = t82716 * t7611;
    let t89327 = F::cast_from(0.18277045187202515961e-2_f64) * t82822 * t25550;
    let t89329 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25476;
    let t89360 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25467;
    let t89362 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25459;
    let t89366 = t82632 * t7604;
    let t89369 = F::cast_from(0.24369393582936687948e-2_f64) * t88810 * t25723;
    (t89310, t89327, t89329, t89360, t89362, t89366, t89369)
}
