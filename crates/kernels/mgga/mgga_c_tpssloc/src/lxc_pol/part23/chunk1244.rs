//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1244/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1244<F: Float>(t77037: F, t77082: F, t77097: F, t77114: F, t893: F, t913: F, t5791: F, t5811: F, t959: F, t13727: F, t21315: F, t2842: F, t5695: F, t5726: F, t1557: F, t21299: F, t2792: F) -> (F, F, F, F, F) {
    let t77119 = 1.0 * t893 * (t77037 + t77082 + t77097 + t77114) * t913;
    let t77122 = 0.21053605041484726346e2 * t959 * t5811 * t5791;
    let t77124 = 24.0 * t13727 * t21315;
    let t77127 = 36.0 * t2842 * t5695 * t5726;
    let t77130 = 8.0 * t2792 * t1557 * t21299;
    (t77119, t77122, t77124, t77127, t77130)
}
