//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1198/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1198<F: Float>(t232: F, t76073: F, t5584: F, t40933: F, t9975: F, t13251: F, t13262: F, t1484: F, t16839: F, t16891: F, t20885: F, t20887: F, t20972: F, t2632: F, t2643: F, t2645: F, t4178: F, t4180: F, t5527: F, t5591: F, t5617: F, t67607: F, t67612: F, t67625: F, t67637: F, t67639: F, t68246: F, t9646: F) -> (F, F, F, F, F) {
    let t76074 = t76073 * t232;
    let t76085 = t5584 * t5584;
    let t76086 = t76085 * t40933;
    let t76090 = t76085 * t9975;
    let t76132 = t2643 * t2645 * t67607 * t5591 / 192.0 - 7.0 / 48.0 * t67612 + 7.0 / 48.0 * t67625 - 5.0 / 128.0 * t2643 * t9646 * t16839 * t20972 + t13262 * t2645 * t67607 * t9975 * t1484 / 32.0 - 3.0 / 256.0 * t13262 * t4180 * t16839 * t68246 + 5.0 / 64.0 * t4178 * t9646 * t16839 * t2632 * t5527 + 35.0 / 96.0 * t67637 + 7.0 / 384.0 * t67639 + t2643 * t2645 * t16891 * t20885 / 128.0 + t13251 * t20887 / 64.0 - t2643 * t4180 * t16891 * t5617 / 512.0;
    (t76074, t76085, t76086, t76090, t76132)
}
