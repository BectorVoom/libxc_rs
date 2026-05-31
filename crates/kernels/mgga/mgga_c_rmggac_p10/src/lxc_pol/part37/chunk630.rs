//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 630/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk630<F: Float>(t15904: F, t15881: F, t515: F, t235: F, t15303: F, t15307: F, t15331: F, t15337: F, t15342: F, t15345: F, t15348: F, t15351: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15905 = F::cast_from(0.2363e1_f64) * t15904;
    let t15907 = t515 * t15881;
    let t15908 = t235 * t15907;
    let t15909 = F::cast_from(0.19957069503106347607e-1_f64) * t15908;
    let t15910 = F::cast_from(0.93188427318671584242e-2_f64) * t15303;
    let t15911 = F::cast_from(0.15531404553111930707e-1_f64) * t15307;
    let t15914 = F::cast_from(0.58171619854173713844e-5_f64) * t15331;
    let t15915 = F::cast_from(0.87596530464506835932e-6_f64) * t15337;
    let t15916 = F::cast_from(0.87596530464506835932e-6_f64) * t15342;
    let t15917 = F::cast_from(0.17519306092901367187e-6_f64) * t15345;
    let t15918 = F::cast_from(0.43798265232253417968e-6_f64) * t15348;
    let t15919 = F::cast_from(0.35038612185802734374e-6_f64) * t15351;
    (t15905, t15907, t15909, t15910, t15911, t15914, t15915, t15916, t15917, t15918, t15919)
}
