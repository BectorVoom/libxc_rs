//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 837/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk837<F: Float>(t30: F, t821: F, t1692: F, t1713: F, t2439: F, t5539: F, t5586: F, t5590: F, t580: F, t331: F, t588: F, t136: F, t1718: F, t921: F, t1717: F, t762: F) -> (F, F, F, F, F, F) {
    let t5591 = t30 * t821;
    let t5598 = 3.0 / 2.0 * t2439 * t1713 * t5539 + t1692 * t5586 * t30 / 2.0 - t1692 * t5590 * t5591 / 2.0 + t1692 * t1713 * t580 / 2.0;
    let t5599 = t588 * t331;
    let t5600 = t5599 * t136;
    let t5604 = t1718 * t921 / 288.0;
    let t5605 = t1717 * t762;
    (t5591, t5598, t5599, t5600, t5604, t5605)
}
