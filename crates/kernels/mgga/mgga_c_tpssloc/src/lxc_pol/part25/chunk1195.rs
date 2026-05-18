//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1195/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1195<F: Float>(t80956: F, t80970: F, t80943: F, t80947: F, t80950: F, t80959: F, t80963: F, t80974: F, t80978: F, t80982: F, t80985: F, t80987: F, t80989: F, t80992: F, t80994: F, t80998: F, t81001: F, t81003: F, t81005: F, t81007: F) -> F {
    let t84555 = F::new(0.13958506597733353653e-1) * t80956;
    let t84558 = F::new(0.87474304870637513515e-3) * t80970;
    let t84572 = -F::new(0.16956557559538964158e-1) * t80943 + F::new(0.72670960969452703536e-2) * t80947 - F::new(0.24223653656484234512e-2) * t80950 - t84555 - F::new(0.10173934535723378495e0) * t80959 - F::new(0.50869672678616892475e-1) * t80963 + t84558 - F::new(0.14534192193890540707e-1) * t80974 + F::new(0.72670960969452703536e-2) * t80978 + F::new(0.72670960969452703536e-2) * t80982 + F::new(0.24223653656484234512e-2) * t80985 - t80987 / F::new(768.0) + F::new(7.0) / F::new(384.0) * t80989 + F::new(7.0) / F::new(192.0) * t80992 - t80994 / F::new(256.0) - F::new(7.0) / F::new(192.0) * t80998 + t81001 / F::new(128.0) - t81003 / F::new(256.0) - t81005 / F::new(768.0) + F::new(7.0) / F::new(384.0) * t81007;
    t84572
}
