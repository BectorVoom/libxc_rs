//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1219/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1219<F: Float>(t63957: F, t63960: F, t63964: F, t63966: F, t61063: F, t61065: F, t61073: F, t62711: F, t63951: F, t63953: F, t63955: F, t63962: F, t63968: F, t63973: F, t63977: F, t63990: F) -> (F, F, F, F) {
    let t66418 = 35.0 / 108.0 * t63957;
    let t66420 = 7.0 / 144.0 * t63960;
    let t66422 = 119.0 / 864.0 * t63964;
    let t66423 = 7.0 / 36.0 * t63966;
    let t66425 = -35.0 / 54.0 * t61063 + 7.0 / 72.0 * t61065 - t63951 / 48.0 + t63953 / 192.0 + t63955 / 384.0 - t66418 - 7.0 / 24.0 * t61073 + t66420 - t63962 / 192.0 - t66422 - t62711 + t66423 - t63968 / 24.0;
    let t66427 = 7.0 / 576.0 * t63973;
    let t66429 = 35.0 / 144.0 * t63977;
    let t66434 = 7.0 / 12.0 * t63990;
    (t66425, t66427, t66429, t66434)
}
