//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2236/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2236<F: Float>(t40817: F, t157: F, t41279: F, t4196: F, t4205: F, t9868: F, t13130: F, t2427: F, t41251: F, t10121: F, t13191: F, t1877: F, t2523: F, t39563: F, t39585: F, t39590: F, t39593: F, t4307: F, t4314: F) -> (F, F, F, F, F, F) {
    let t46331 = F::cast_from(0.51947577317044391277e2_f64) * t40817;
    let t46334 = F::cast_from(36.0_f64) * t41279 * t157 * t4196;
    let t46335 = t4205 * t9868;
    let t46336 = F::cast_from(12.0_f64) * t46335;
    let t46338 = F::cast_from(12.0_f64) * t2427 * t13130;
    let t46339 = F::cast_from(12.0_f64) * t41251;
    let t46340 = -t10121 * t1877 * t4307 + F::cast_from(36.0_f64) * t13191 * t2523 * t4314 + t39563 - t39585 + t39590 - t39593 - t46331 + t46334 + t46336 + t46338 + t46339;
    (t46331, t46334, t46336, t46338, t46339, t46340)
}
