//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 538/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk538<F: Float>(t1012: F, t6754: F, t1030: F, t1940: F, t354: F, t1036: F, t1942: F, t1039: F, t1000: F, t1025: F, t1046: F, t1935: F, t1937: F, t350: F, t378: F, t6712: F, t6716: F, t6717: F, t6723: F, t6728: F, t6730: F, t6735: F, t6742: F, t6747: F, t6750: F) -> (F, F, F, F, F, F) {
    let t6755 = t1012 * t6754;
    let t6758 = t1940 * t1030;
    let t6759 = t354 * t6758;
    let t6763 = t1942 * t1036 / 2304.0;
    let t6764 = t1940 * t1039;
    let t6765 = t354 * t6764;
    let t6768 = -t6712 * t350 / 36.0 + t6716 + t6717 * t1000 / 288.0 - 0.80745512188280781712e-3 * t6723 * t1937 + t6728 + 0.10093189023535097714e-3 * t6730 * t1937 - 0.10093189023535097714e-3 * t1935 * t6735 + 0.10093189023535097714e-3 * t6742 * t6747 + t6750 * t378 / 1536.0 + t6755 * t1025 / 1536.0 - t6759 * t378 / 288.0 + t6763 + t6765 * t1046 / 2304.0;
    (t6755, t6758, t6759, t6764, t6765, t6768)
}
