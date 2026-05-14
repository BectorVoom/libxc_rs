//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1065/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1065<F: Float>(t24744: F, t24746: F, t2132: F, t23413: F, t3523: F, t7345: F, t3572: F, t7339: F, t1218: F, t1232: F, t2134: F, t2136: F, t24704: F, t24706: F, t24712: F, t24716: F, t24723: F, t24729: F, t24733: F, t24736: F, t24741: F, t3496: F, t3511: F, t3518: F, t3527: F, t3531: F, t3580: F) -> (F, F) {
    let t24747 = t24744 * t24746;
    let t24749 = t2132 * t23413;
    let t24752 = t7345 * t3523;
    let t24754 = t7339 * t3572;
    let t24756 = -t24704 - 0.10093189023535097714e-3 * t2134 * t24706 - t7345 * t3527 / 2304.0 - 0.20186378047070195428e-3 * t24712 - t7345 * t3531 / 1152.0 + t24716 * t1218 / 768.0 + 0.20186378047070195428e-3 * t24723 + t7339 * t3496 / 1536.0 + t24729 * t3511 / 768.0 - t24733 * t3518 / 1536.0 - t24736 * t1232 / 1152.0 - t24741 * t3580 / 1152.0 - 0.20186378047070195428e-3 * t24747 - 0.10093189023535097714e-3 * t24749 * t2136 - t24752 / 1728.0 + t24754 / 1152.0;
    (t24749, t24756)
}
