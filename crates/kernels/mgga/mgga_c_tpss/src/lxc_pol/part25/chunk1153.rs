//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1153/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1153<F: Float>(t33: F, t4706: F, t18246: F, t21262: F, t1364: F, t1497: F, t4701: F, t4806: F, t1398: F, t4802: F, t1600: F, t6323: F, t1812: F, t21255: F, t18737: F, t18746: F, t19693: F, t19706: F, t19718: F, t21274: F, t21276: F, t21278: F, t21280: F, t21282: F, t21284: F, t21286: F, t21288: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21485 = t33 * t4706;
    let t21492 = t18246 * t21262;
    let t21495 = t1497 * t1364;
    let t21499 = t33 * t4701;
    let t21510 = t33 * t4806;
    let t21513 = t1497 * t1398;
    let t21516 = t33 * t4802;
    let t21576 = t1600 * t6323;
    let t21583 = t1812 * t21255;
    let t21608 = t18737 + 7.0 / 36.0 * t19693 + t21274 / 8.0 - t21276 / 24.0 + t21278 / 384.0 + 7.0 / 576.0 * t19706 + t21280 / 96.0 - t21282 / 768.0 - t21284 / 768.0 + t18746 + 7.0 / 144.0 * t19718 + 5.0 / 192.0 * t21286 - t21288 / 192.0;
    (t21485, t21492, t21495, t21499, t21510, t21513, t21516, t21576, t21583, t21608)
}
