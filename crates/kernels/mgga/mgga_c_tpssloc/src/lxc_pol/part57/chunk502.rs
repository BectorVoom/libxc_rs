//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 502/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk502<F: Float>(t3034: F, t334: F, t1930: F, t1934: F, t344: F, t1009: F, t1014: F, t363: F, t1018: F, t1012: F, t1036: F, t1942: F, t1039: F, t1940: F, t354: F, t1946: F, t225: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6739 = 1.0 / t3034 / t334;
    let t6740 = t1930 * t6739;
    let t6741 = t1934 * t344;
    let t6742 = t6740 * t6741;
    let t6743 = t1009 * t1014;
    let t6744 = t6743 * t363;
    let t6753 = t1014 * sigma0;
    let t6754 = t6753 * t1018;
    let t6755 = t1012 * t6754;
    let t6763 = t1942 * t1036 / 2304.0;
    let t6764 = t1940 * t1039;
    let t6765 = t354 * t6764;
    let t6771 = t1946 * t225;
    (t6740, t6741, t6742, t6743, t6744, t6753, t6754, t6755, t6763, t6764, t6765, t6771)
}
