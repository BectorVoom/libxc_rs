//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 563/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk563<F: Float>(t14530: F, t82: F, t72: F, t13851: F, t13854: F, t13856: F, t13859: F, t13864: F, t13869: F, t13873: F, t13877: F, t13881: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14531 = t82 * t14530;
    let t14532 = t72 * t14531;
    let t14533 = F::cast_from(0.1276937996798935182e-4_f64) * t13851;
    let t14534 = F::cast_from(0.58171619854173713846e-5_f64) * t13854;
    let t14535 = F::cast_from(0.85129199786595678799e-5_f64) * t13856;
    let t14536 = F::cast_from(0.85129199786595678799e-5_f64) * t13859;
    let t14537 = F::cast_from(0.17519306092901367188e-6_f64) * t13864;
    let t14538 = F::cast_from(0.15961724959986689775e-4_f64) * t13869;
    let t14539 = F::cast_from(0.1276937996798935182e-4_f64) * t13873;
    let t14540 = F::cast_from(0.2553875993597870364e-4_f64) * t13877;
    let t14541 = F::cast_from(0.3830813990396805546e-4_f64) * t13881;
    (t14531, t14532, t14533, t14534, t14535, t14536, t14537, t14538, t14539, t14540, t14541)
}
