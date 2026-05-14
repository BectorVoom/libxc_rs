//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1236/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1236<F: Float>(t1713: F, t4706: F, t1364: F, t1398: F, t1692: F, t18052: F, t198: F, t19802: F, t207: F, t21262: F, t21344: F, t2439: F, t3552: F, t4701: F, t4802: F, t4806: F, t5590: F, t6149: F, t823: F) -> (F, F) {
    let t21453 = t1713 * t4706;
    let t21476 = t198 * t207 * t21344 * t823 + 6.0 * t1364 * t2439 * t6149 - 2.0 * t1398 * t1692 * t19802 + 2.0 * t1692 * t18052 * t4806 - t1692 * t4802 * t5590 + 3.0 * t1713 * t2439 * t4701 - 6.0 * t21262 * t2439 * t5590 + 6.0 * t21453 * t3552;
    (t21453, t21476)
}
