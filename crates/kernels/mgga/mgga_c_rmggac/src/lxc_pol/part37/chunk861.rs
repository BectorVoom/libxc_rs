//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 861/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk861<F: Float>(t76054: F, t70358: F, t76066: F, t70365: F, t70369: F, t70373: F, t15675: F, t4965: F, t70381: F, t76079: F, t76087: F, t76090: F, t70387: F, t76110: F, t30204: F, t78220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78475 = 0.20455996240684006298e-1 * t76054;
    let t78476 = 0.86737941314158990619e-4 * t70358;
    let t78477 = 0.14967802127329760705e-1 * t76066;
    let t78478 = 0.30487649791575028312e-3 * t70365;
    let t78479 = 0.43368970657079495308e-4 * t70369;
    let t78480 = 0.30487649791575028312e-3 * t70373;
    let t78482 = 0.11974241701863808564e0 * t4965 * t15675;
    let t78483 = 0.16263363996404810741e-4 * t70381;
    let t78484 = 0.13637330827122670865e-1 * t76079;
    let t78486 = 0.5107751987195740728e-4 * t76087;
    let t78487 = 0.2553875993597870364e-4 * t76090;
    let t78488 = 0.38430329123504567781e-4 * t70387;
    let t78491 = 0.14967802127329760705e-1 * t76110;
    let t78493 = 0.23948483403727617128e0 * t30204 * t78220;
    (t78475, t78476, t78477, t78478, t78479, t78480, t78482, t78483, t78484, t78486, t78487, t78488, t78491, t78493)
}
