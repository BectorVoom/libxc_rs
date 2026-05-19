//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 547/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk547<F: Float>(t14392: F, t352: F, t698: F, t875: F, t1971: F, t3351: F, t13799: F, t13803: F, t13813: F, t13820: F, t13829: F, t270: F, t703: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14393 = F::cast_from(0.12769379967989351819e-4_f64) * t14392;
    let t14394 = t698 * t352;
    let t14395 = t875 * t14394;
    let t14396 = t1971 * t14395;
    let t14397 = t3351 * t14396;
    let t14398 = F::cast_from(0.85129199786595678796e-5_f64) * t14397;
    let t14399 = F::cast_from(0.20455996240684006296e-1_f64) * t13799;
    let t14400 = F::cast_from(0.40911992481368012592e-1_f64) * t13803;
    let t14404 = F::cast_from(0.72042316457491791901e-3_f64) * t13813;
    let t14406 = F::cast_from(0.38430329123504567781e-4_f64) * t13820;
    let t14408 = F::cast_from(0.68186654135613354325e-2_f64) * t13829;
    let t14413 = t703 * t270;
    (t14393, t14396, t14398, t14399, t14400, t14404, t14406, t14408, t14413)
}
