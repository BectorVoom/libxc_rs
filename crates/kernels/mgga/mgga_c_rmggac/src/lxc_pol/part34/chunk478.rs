//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 478/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk478<F: Float>(t14174: F, t1550: F, t495: F, t664: F, t515: F, t1971: F, t7230: F, t498: F, t7231: F, t3351: F, t7799: F, t3352: F, t2144: F, t7879: F, t13957: F, t875: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14175 = t1550 * t14174;
    let t14178 = t664 * t495;
    let t14179 = t515 * t14178;
    let t14180 = t1971 * t14179;
    let t14181 = t7230 * t14180;
    let t14182 = 0.1064114997332445985e-4 * t14181;
    let t14183 = t664 * t498;
    let t14184 = t515 * t14183;
    let t14185 = t7231 * t14184;
    let t14186 = t3351 * t14185;
    let t14188 = t515 * t7799;
    let t14189 = t3352 * t14188;
    let t14190 = t3351 * t14189;
    let t14192 = t2144 * t7879;
    let t14193 = t1971 * t14192;
    let t14194 = t3351 * t14193;
    let t14198 = t875 * t13957;
    (t14175, t14180, t14182, t14185, t14186, t14189, t14190, t14193, t14194, t14198)
}
