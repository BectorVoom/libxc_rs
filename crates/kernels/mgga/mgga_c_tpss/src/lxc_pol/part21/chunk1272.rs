//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1272/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1272<F: Float>(t1678: F, t61942: F, t2003: F, t48: F, t2009: F, t588: F, t43: F, t7737: F, t789: F, t582: F, t7682: F, t18351: F, t5502: F, t1679: F, t1982: F, t1981: F, t1993: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61943 = t1678 * t61942;
    let t61961 = t2003 * t48;
    let t61964 = t588 * t2009;
    let t61969 = t43 * t7737;
    let t61976 = 1232.0 / 27.0 * t789;
    let t62007 = t7682 * t582;
    let t62010 = t5502 * t18351;
    let t62020 = t1679 * t1982;
    let t62021 = t1678 * t62020;
    let t62024 = t1981 * t1993;
    (t61943, t61961, t61964, t61969, t61976, t62007, t62010, t62021, t62024)
}
