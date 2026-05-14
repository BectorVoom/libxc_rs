//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 503/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk503<F: Float>(t1356: F, t14427: F, t14276: F, t14278: F, t14280: F, t2228: F, t36: F, t305: F, t664: F, t8264: F, t118: F, t698: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14428 = t1356 * t14427;
    let t14429 = 0.11974241701863808564e0 * t14428;
    let t14431 = 0.20455996240684006298e-1 * t14276;
    let t14432 = 0.2727466165424534173e-1 * t14278;
    let t14433 = 0.13637330827122670865e-1 * t14280;
    let t14438 = t2228 * t36;
    let t14439 = t305 * t14438;
    let t14440 = 0.14967802127329760705e-1 * t14439;
    let t14441 = t8264 * t664;
    let t14443 = 0.39914139006212695214e-1 * t118 * t14441;
    let t14444 = t698 * t664;
    (t14429, t14431, t14432, t14433, t14438, t14440, t14441, t14443, t14444)
}
