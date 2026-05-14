//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 502/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk502<F: Float>(t13813: F, t13820: F, t13829: F, t270: F, t703: F, t2039: F, t638: F, t31: F, t2046: F, t2050: F, t2211: F, t7799: F, t739: F, t7879: F, t884: F, t13957: F, t8041: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14404 = 0.72042316457491791901e-3 * t13813;
    let t14406 = 0.38430329123504567781e-4 * t13820;
    let t14408 = 0.68186654135613354325e-2 * t13829;
    let t14413 = t703 * t270;
    let t14415 = t638 * t2039 * t14413;
    let t14417 = t703 * t31;
    let t14419 = t2046 * t2050 * t14417;
    let t14421 = t2211 * t7799;
    let t14422 = t739 * t14421;
    let t14423 = 0.11974241701863808564e0 * t14422;
    let t14424 = t2211 * t7879;
    let t14425 = t884 * t14424;
    let t14426 = 0.11974241701863808564e0 * t14425;
    let t14427 = t8041 * t13957;
    (t14404, t14406, t14408, t14413, t14415, t14417, t14419, t14421, t14423, t14424, t14426, t14427)
}
