//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1327/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1327<F: Float>(t1364: F, t19818: F, t8096: F, t19809: F, t44169: F, t18246: F, t69799: F, t20047: F, t70240: F, t69881: F, t1006: F, t4806: F) -> (F, F, F, F, F, F) {
    let t70759 = t8096 * t1364 * t19818;
    let t70771 = t44169 * t19809;
    let t70800 = t18246 * t69799;
    let t70803 = t20047 * t70240;
    let t70805 = t20047 * t69881;
    let t70808 = t1006 * t4806;
    (t70759, t70771, t70800, t70803, t70805, t70808)
}
