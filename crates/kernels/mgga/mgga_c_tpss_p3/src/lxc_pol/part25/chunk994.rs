//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 994/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk994<F: Float>(t3267: F, t5410: F, t12891: F, t13677: F, t13682: F, t13687: F, t13691: F, t13695: F, t13700: F, t13703: F, t13707: F, t13711: F, t13715: F, t13719: F, t3271: F, t4413: F) -> F {
    let t13722 = t3267 * t5410;
    let t13724 = t3271 * t13677 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3271 * t13682 + t3271 * t13687 / F::cast_from(768.0_f64) - t3271 * t13691 / F::cast_from(1536.0_f64) - t3271 * t13695 / F::cast_from(3072.0_f64) + t4413 * t13700 / F::cast_from(1536.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t13703 - t12891 * t13707 / F::cast_from(512.0_f64) + t4413 * t13711 / F::cast_from(512.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3271 * t13715 + t3271 * t13719 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t13722;
    t13724
}
