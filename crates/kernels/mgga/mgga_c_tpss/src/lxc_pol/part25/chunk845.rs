//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 845/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk845<F: Float>(t7921: F, t7997: F, t162: F, t158: F, t2206: F, t2218: F, t713: F, t720: F, t7870: F, t735: F, t2214: F, t7813: F, t7857: F, t2332: F, t692: F, t2210: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7998 = t7921 + t7997;
    let t7999 = t162 * t7998;
    let t8000 = t158 * t7999;
    let t8006 = t2218 * t2206;
    let t8017 = t713 * t7870 * t720;
    let t8019 = 0.5848223622634646207e0 * t735 * t8017;
    let t8021 = t7857 * t7813 * t2214;
    let t8023 = 0.10389515463408878255e3 * t735 * t8021;
    let t8024 = t692 * t2332;
    let t8027 = t2210 * t7813 * t720;
    (t7998, t8000, t8006, t8017, t8019, t8021, t8023, t8024, t8027)
}
