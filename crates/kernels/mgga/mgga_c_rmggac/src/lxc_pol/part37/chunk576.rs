//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 576/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk576<F: Float>(t14935: F, t338: F, t118: F, t14338: F, t14333: F, t14335: F, t14354: F, t14521: F, t14523: F, t14524: F, t14527: F, t14969: F, t14977: F, t14981: F, t305: F, t326: F) -> (F, F, F, F) {
    let t15001 = t338 * t14935;
    let t15002 = t118 * t15001;
    let t15007 = F::new(0.16566831523319392754e-1) * t14338;
    let t15012 = -t14333 + t14335 + F::new(0.59871208509319042821e-1) * t305 * t14969 - t14521 + t15007 - F::new(0.59871208509319042821e-1) * t326 * t14977 - F::new(0.39914139006212695214e-1) * t118 * t14981 + t14523 + t14524 + t14354 + t14527;
    (t15001, t15002, t15007, t15012)
}
