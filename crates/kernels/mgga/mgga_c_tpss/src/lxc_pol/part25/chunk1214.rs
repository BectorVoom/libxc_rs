//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1214/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1214<F: Float>(t18246: F, t69799: F, t20047: F, t70240: F, t69881: F, t1006: F, t4806: F, t69863: F, t4802: F, t64879: F, t70243: F, t4701: F, t1497: F, t3683: F, t823: F, t21262: F, t61703: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t70800 = t18246 * t69799;
    let t70803 = t20047 * t70240;
    let t70805 = t20047 * t69881;
    let t70808 = t1006 * t4806;
    let t70813 = t18246 * t69863;
    let t70816 = t1006 * t4802;
    let t70828 = t64879 * t70243;
    let t70839 = t1006 * t4701;
    let t70844 = t823 * t1497 * t3683;
    let t70847 = t61703 * t21262;
    (t70800, t70803, t70805, t70808, t70813, t70816, t70828, t70839, t70844, t70847)
}
