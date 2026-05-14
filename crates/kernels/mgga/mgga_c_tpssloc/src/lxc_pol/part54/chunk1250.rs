//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1250/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1250<F: Float>(t113934: F, t115292: F, t115294: F, t120180: F, t120184: F, t120196: F, t122102: F, t122107: F, t122110: F, t122112: F, t122117: F, t2092: F, t24082: F, t7750: F, t90732: F, t115352: F, t6897: F, t7700: F) -> (F, F) {
    let t122119 = t120180 + t120184 - t90732 * t2092 - 0.38381794893125283518e-1 * t122102 - t24082 * t7750 + t113934 + 0.19190897446562641759e-1 * t115292 + 0.16449340668482264365e-1 * t122107 + 0.16449340668482264365e-1 * t122110 - 0.38381794893125283518e-1 * t122112 + 0.19190897446562641759e-1 * t115294 - t120196 + 0.16449340668482264365e-1 * t122117;
    let t122121 = t6897 * t115352 * t7700;
    (t122119, t122121)
}
