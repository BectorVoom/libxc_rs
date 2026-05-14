//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 216/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk216<F: Float>(t45: F, t57: F, t190: F, t581: F, t681: F, t78: F, t81: F, t150: F, t169: F, t164: F, t662: F, t664: F, t668: F, t673: F, t172: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t682 = t190 * t581;
    let t684 = 4.0 * t681 * t682;
    let t687 = piecewise3(t151, 0.0, 4.0 / 3.0 * t78 * t581);
    let t690 = piecewise3(t155, 0.0, -4.0 / 3.0 * t81 * t581);
    let t691 = t687 + t690;
    let t692 = t150 * t691;
    let t693 = t692 * t190;
    let t697 = t169 * t169;
    let t698 = 1.0 / t697;
    let t699 = t164 * t698;
    let t704 = -0.1176575e1 * t662 - 0.516475e0 * t664 - 0.2103875e0 * t668 - 0.104195e0 * t673;
    let t705 = 1.0 / t172;
    (t682, t684, t691, t692, t693, t697, t698, t699, t704, t705)
}
