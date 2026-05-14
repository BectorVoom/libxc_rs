//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1284/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1284<F: Float>(t7769: F, t84033: F, t20173: F, t33659: F, t3941: F, t7056: F, t7467: F, t24462: F, t1458: F, t7263: F, t2113: F, t4072: F, t119931: F, t2108: F, t2240: F, t131: F, t27331: F) -> (F, F, F, F, F, F, F, F) {
    let t122839 = 27.0 * t84033 * t7769;
    let t122841 = 27.0 * t20173 * t33659;
    let t122844 = 27.0 * t3941 * t7056 * t7467;
    let t122846 = 0.135e2 * t24462 * t7467;
    let t122917 = t7263 * t1458;
    let t122920 = t2113 * t4072;
    let t122941 = t2240 * t119931 * t2108;
    let t122945 = t2240 * t27331 * t131;
    (t122839, t122841, t122844, t122846, t122917, t122920, t122941, t122945)
}
