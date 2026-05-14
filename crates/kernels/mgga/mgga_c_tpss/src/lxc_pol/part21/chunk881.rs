//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 881/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk881<F: Float>(t259: F, t379: F, t1364: F, t1713: F, t207: F, t6148: F, t1398: F, t1692: F, t198: F, t2439: F, t5590: F, t823: F, t1485: F, t330: F, t4023: F, t5652: F, t6185: F, t995: F) -> (F, F, F) {
    let t380 = t259 < t379;
    let t6192 = t1713 * t1364;
    let t6195 = t207 * t6148;
    let t6200 = -t1398 * t1692 * t5590 + t198 * t6195 * t823 + 3.0 * t2439 * t6192;
    let t6201 = piecewise3(t380, t198 * t330 * t6185 * t995 - t1485 * t4023 * t5652, t6200);
    (t6192, t6200, t6201)
}
