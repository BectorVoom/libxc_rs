//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 501/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk501<F: Float>(t259: F, t379: F, t1727: F, t1730: F, t1733: F, t373: F, t1712: F, t207: F, t198: F, t823: F, t330: F, t995: F) -> (F, F, F, F) {
    let t380 = t259 < t379;
    let t1735 = t1727 * t373 - t1730 * t1733;
    let t1739 = t207 * t1712;
    let t1741 = t198 * t1739 * t823;
    let t1742 = piecewise3(t380, t198 * t330 * t1735 * t995, t1741);
    (t1735, t1739, t1741, t1742)
}
