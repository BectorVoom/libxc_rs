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
    let t13724 = t3271 * t13677 / F::new(768.0) - F::new(5.0) / F::new(768.0) * t3271 * t13682 + t3271 * t13687 / F::new(768.0) - t3271 * t13691 / F::new(1536.0) - t3271 * t13695 / F::new(3072.0) + t4413 * t13700 / F::new(1536.0) - F::new(7.0) / F::new(576.0) * t13703 - t12891 * t13707 / F::new(512.0) + t4413 * t13711 / F::new(512.0) - F::new(5.0) / F::new(384.0) * t3271 * t13715 + t3271 * t13719 / F::new(384.0) + F::new(7.0) / F::new(4608.0) * t13722;
    t13724
}
