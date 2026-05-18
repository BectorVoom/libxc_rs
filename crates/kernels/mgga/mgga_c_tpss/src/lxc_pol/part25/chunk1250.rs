//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1250/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1250<F: Float>(t30: F, t4806: F, t1288: F, t1398: F, t4802: F, t33: F, t4706: F, t18246: F, t21262: F, t1364: F, t1497: F, t4701: F) -> (F, F, F, F, F, F, F) {
    let t21353 = t30 * t4806;
    let t21356 = t1288 * t1398;
    let t21359 = t30 * t4802;
    let t21485 = t33 * t4706;
    let t21492 = t18246 * t21262;
    let t21495 = t1497 * t1364;
    let t21499 = t33 * t4701;
    (t21353, t21356, t21359, t21485, t21492, t21495, t21499)
}
