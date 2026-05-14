//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 502/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk502<F: Float>(t275: F, t3208: F, t13813: F, t13820: F, t13825: F, t13829: F, t13833: F, t13837: F, t13842: F, t270: F, t703: F, t2039: F, t638: F, t31: F, t2046: F, t2050: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14402 = t275 * t3208;
    let t14404 = 0.72042316457491791901e-3 * t13813;
    let t14406 = 0.38430329123504567781e-4 * t13820;
    let t14407 = 0.76860658247009135557e-5 * t13825;
    let t14408 = 0.68186654135613354325e-2 * t13829;
    let t14409 = 0.93188427318671584245e-2 * t13833;
    let t14410 = 0.15531404553111930708e-1 * t13837;
    let t14411 = 0.31062809106223861415e-2 * t13842;
    let t14413 = t703 * t270;
    let t14415 = t638 * t2039 * t14413;
    let t14417 = t703 * t31;
    let t14419 = t2046 * t2050 * t14417;
    (t14402, t14404, t14406, t14407, t14408, t14409, t14410, t14411, t14413, t14415, t14417, t14419)
}
