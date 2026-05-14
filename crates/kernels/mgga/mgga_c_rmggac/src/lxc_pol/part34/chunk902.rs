//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 902/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk902<F: Float>(t76087: F, t76090: F, t70387: F, t76110: F, t30204: F, t78220: F, t26291: F, t78223: F, t40724: F, t78070: F, t76113: F, t76116: F, t76119: F, t76122: F, t76127: F, t76130: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78486 = 0.5107751987195740728e-4 * t76087;
    let t78487 = 0.2553875993597870364e-4 * t76090;
    let t78488 = 0.38430329123504567781e-4 * t70387;
    let t78491 = 0.14967802127329760705e-1 * t76110;
    let t78493 = 0.23948483403727617128e0 * t30204 * t78220;
    let t78495 = 0.35922725105591425692e0 * t26291 * t78223;
    let t78497 = 0.35922725105591425692e0 * t40724 * t78070;
    let t78498 = 0.44903406381989282115e-1 * t76113;
    let t78499 = 0.2993560425465952141e-1 * t76116;
    let t78500 = 0.17961362552795712846e0 * t76119;
    let t78501 = 0.44903406381989282115e-1 * t76122;
    let t78502 = 0.30487649791575028312e-3 * t76127;
    let t78503 = 0.72042316457491791901e-3 * t76130;
    (t78486, t78487, t78488, t78491, t78493, t78495, t78497, t78498, t78499, t78500, t78501, t78502, t78503)
}
