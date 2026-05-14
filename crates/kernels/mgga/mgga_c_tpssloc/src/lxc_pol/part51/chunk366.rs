//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 366/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk366<F: Float>(t1730: F, t484: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F, t475: F, t1214: F, t248: F, t46: F, t480: F, t47: F, t479: F, t471: F, t1230: F, t1653: F) -> (F, F, F, F, F, F, F, F) {
    let t1731 = t1730 * t484;
    let t1734 = -t1659 + t1673 + t1699 + t1701 - t1705;
    let t1735 = t1734 * t475;
    let t1737 = t248 * t1214 * t1735;
    let t1740 = t480 * t46;
    let t1742 = 1.0 / t47 / t1740;
    let t1743 = t479 * t1742;
    let t1744 = t471 * t1743;
    let t1748 = t248 * t1230 * t1653;
    (t1731, t1734, t1735, t1737, t1742, t1743, t1744, t1748)
}
