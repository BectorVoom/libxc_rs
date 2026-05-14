//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1187/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1187<F: Float>(t6353: F, t8096: F, t198: F, t206: F, t6337: F, t768: F, t63907: F, t63913: F, t63917: F, t63928: F, t63960: F, t63966: F, t63973: F, t63977: F, t63990: F, t1395: F, t18770: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t66299 = t6353 * t8096;
    let t66317 = t198 * t206 * t6353;
    let t66362 = t768 * t6337;
    let t66390 = 7.0 / 144.0 * t63907;
    let t66393 = 7.0 / 144.0 * t63913;
    let t66394 = 7.0 / 288.0 * t63917;
    let t66399 = 7.0 / 576.0 * t63928;
    let t66420 = 7.0 / 144.0 * t63960;
    let t66423 = 7.0 / 36.0 * t63966;
    let t66427 = 7.0 / 576.0 * t63973;
    let t66429 = 35.0 / 144.0 * t63977;
    let t66434 = 7.0 / 12.0 * t63990;
    let t66480 = t18770 * t1395;
    (t66299, t66317, t66362, t66390, t66393, t66394, t66399, t66420, t66423, t66427, t66429, t66434, t66480)
}
