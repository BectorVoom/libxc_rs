//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1237/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1237<F: Float>(t1985: F, t7700: F, t97511: F, t1842: F, t22635: F, t26331: F, t96922: F, t1992: F, t26354: F, t6460: F, t22633: F, t97637: F, t1375: F, t20029: F, t2015: F, t2016: F, t20613: F, t20661: F, t28220: F, t3887: F, t5215: F, t6958: F, t74930: F, t7729: F, t90551: F, t90582: F, t96920: F, t97503: F) -> (F, F, F) {
    let t106986 = t1985 * t97511 * t7700;
    let t106991 = t26331 * t22635 * t96922 * t1842;
    let t107007 = t1992 * t22635 * t26354 * t6460;
    let t107015 = t22633 * t22635 * t97637 * t1842;
    let t107024 = -0.23029076935875170111e0 * t96920 + 6.0 * t6958 * t20613 - 0.15626873635058151147e0 * t90551 + 0.49348022005446793095e-1 * t107007 + 12.0 * t5215 * t28220 - t74930 * t2016 + 0.78134368175290755733e-1 * t90582 + 0.49348022005446793095e-1 * t107015 + 12.0 * t20029 * t7729 - 0.49348022005446793095e-1 * t97503 + 2.0 * t1375 * t3887 * t2015 * t20661;
    (t106986, t106991, t107024)
}
