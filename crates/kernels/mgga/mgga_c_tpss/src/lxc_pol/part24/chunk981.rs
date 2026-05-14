//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 981/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk981<F: Float>(t13685: F, t13705: F, t4415: F, t4417: F, t12816: F, t1640: F, t4478: F, t12822: F, t12823: F, t5387: F, t3267: F, t5410: F, t12891: F, t13677: F, t13682: F, t13687: F, t13691: F, t13695: F, t13700: F, t13703: F, t3271: F, t4413: F) -> (F, F, F, F, F) {
    let t13707 = t4415 * t13685 * t13705;
    let t13711 = t4415 * t13685 * t4417;
    let t13715 = t12816 * t1640 * t4478;
    let t13719 = t12822 * t12823 * t5387;
    let t13722 = t3267 * t5410;
    let t13724 = t3271 * t13677 / 768.0 - 5.0 / 768.0 * t3271 * t13682 + t3271 * t13687 / 768.0 - t3271 * t13691 / 1536.0 - t3271 * t13695 / 3072.0 + t4413 * t13700 / 1536.0 - 7.0 / 576.0 * t13703 - t12891 * t13707 / 512.0 + t4413 * t13711 / 512.0 - 5.0 / 384.0 * t3271 * t13715 + t3271 * t13719 / 384.0 + 7.0 / 4608.0 * t13722;
    (t13707, t13711, t13715, t13719, t13724)
}
