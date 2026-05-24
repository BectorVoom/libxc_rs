//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 692/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk692<F: Float>(t739: F, t9321: F, t1587: F, t699: F, t1652: F, t2211: F, t1356: F, t1664: F, t702: F, t289: F, t1614: F, t884: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9322 = t739 * t9321;
    let t9332 = t699 * t1587;
    let t9333 = t739 * t9332;
    let t9340 = t2211 * t1652;
    let t9341 = t1356 * t9340;
    let t9343 = t1664 * t702;
    let t9344 = t289 * t9343;
    let t9352 = t699 * t1614;
    let t9353 = t884 * t9352;
    (t9322, t9332, t9333, t9340, t9341, t9343, t9344, t9352, t9353)
}
