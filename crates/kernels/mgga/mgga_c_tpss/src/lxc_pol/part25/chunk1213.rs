//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1213/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1213<F: Float>(t4806: F, t821: F, t64247: F, t1288: F, t3724: F, t580: F, t14426: F, t30: F, t3610: F, t14076: F, t63840: F, t1364: F, t19818: F, t8096: F, t19809: F, t44169: F) -> (F, F, F, F, F, F, F, F, F) {
    let t70243 = t4806 * t821;
    let t70244 = t64247 * t70243;
    let t70255 = t1288 * t3724;
    let t70258 = t580 * t4806;
    let t70261 = t30 * t14426;
    let t70286 = t1288 * t3610;
    let t70290 = t63840 * t14076;
    let t70759 = t8096 * t1364 * t19818;
    let t70771 = t44169 * t19809;
    (t70243, t70244, t70255, t70258, t70261, t70286, t70290, t70759, t70771)
}
