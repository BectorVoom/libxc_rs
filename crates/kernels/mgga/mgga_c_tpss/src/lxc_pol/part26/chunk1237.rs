//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1237/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1237<F: Float>(t33: F, t4706: F, t1713: F, t18246: F, t21262: F, t1364: F, t1497: F, t4701: F, t4806: F, t1398: F, t4802: F, t1692: F, t17929: F, t18052: F, t19802: F, t21345: F, t2439: F, t3552: F, t5059: F, t5590: F, t6149: F, t6207: F, t6214: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21485 = t33 * t4706;
    let t21486 = t1713 * t21485;
    let t21492 = t18246 * t21262;
    let t21495 = t1497 * t1364;
    let t21499 = t33 * t4701;
    let t21510 = t33 * t4806;
    let t21513 = t1497 * t1398;
    let t21516 = t33 * t4802;
    let t21523 = 3.0 * t3552 * t21486 + 3.0 * t2439 * t6149 * t6207 - 3.0 * t17929 * t21492 + 3.0 * t2439 * t1713 * t21495 + 3.0 / 2.0 * t2439 * t1713 * t21499 + t1692 * t21345 * t33 / 2.0 - t1692 * t19802 * t6214 + t1692 * t6149 * t1497 + t1692 * t18052 * t21510 - t1692 * t5590 * t21513 - t1692 * t5590 * t21516 / 2.0 + t1692 * t1713 * t5059 / 2.0;
    (t21485, t21486, t21492, t21495, t21499, t21510, t21513, t21516, t21523)
}
