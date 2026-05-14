//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 715/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk715<F: Float>(t23511: F, t363: F, t1011: F, t3040: F, t3131: F, t1014: F, t3030: F, t360: F, t1940: F, t3046: F, t354: F, t1046: F, t1935: F, t23489: F, t23495: F, t23500: F, t23504: F, t23510: F, t3057: F, t3064: F, t6723: F, t6730: F, t6735: F, t6742: F, t6747: F, t6765: F) -> (F,) {
    let t23512 = t23511 * t363;
    let t23513 = t3040 * t1011;
    let t23514 = t23513 * t3131;
    let t23515 = t23512 * t23514;
    let t23518 = t3030 * t1014;
    let t23519 = t23518 * t363;
    let t23520 = t23513 * t360;
    let t23521 = t23519 * t23520;
    let t23528 = t1940 * t3046;
    let t23529 = t354 * t23528;
    let t23532 = 0.20186378047070195428e-3 * t23489 * t6747 - 0.20186378047070195428e-3 * t6730 * t6735 - 0.10093189023535097714e-3 * t1935 * t23495 + 0.16149102437656156342e-2 * t6723 * t6735 + t23500 / 1152.0 + 0.10093189023535097714e-3 * t6742 * t23504 + 0.20186378047070195428e-3 * t23510 * t23515 - 0.10093189023535097714e-3 * t23510 * t23521 + t6765 * t3057 / 2304.0 + 5.0 / 6912.0 * t6765 * t3064 - t23529 * t1046 / 216.0;
    (t23532,)
}
