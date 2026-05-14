//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 856/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk856<F: Float>(t78321: F, t15467: F, t4601: F, t1550: F, t699: F, t8704: F, t75859: F, t75864: F, t75866: F, t75887: F, t1356: F, t41063: F, t8041: F, t41015: F, t70086: F, t71343: F, t8571: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t78322 = 0.44903406381989282115e-1 * t78321;
    let t78323 = t4601 * t15467;
    let t78324 = 0.44903406381989282115e-1 * t78323;
    let t78326 = t1550 * t699 * t8704;
    let t78327 = 0.2993560425465952141e-1 * t78326;
    let t78339 = 0.44903406381989282115e-1 * t75859;
    let t78340 = 0.38430329123504567781e-4 * t75864;
    let t78341 = 0.38430329123504567781e-4 * t75866;
    let t78349 = 0.44903406381989282115e-1 * t75887;
    let t78352 = 0.11974241701863808564e0 * t1356 * t8041 * t41063;
    let t78355 = 0.11974241701863808564e0 * t1356 * t8041 * t41015;
    let t78359 = 0.43368970657079495308e-4 * t70086;
    let t78361 = t8571 * t71343;
    (t78322, t78324, t78327, t78339, t78340, t78341, t78349, t78352, t78355, t78359, t78361)
}
