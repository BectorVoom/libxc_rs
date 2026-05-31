//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 214/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk214<F: Float>(t45: F, t57: F, t190: F, t581: F, t681: F, t78: F, t81: F, t150: F, t169: F, t164: F, t662: F, t664: F, t668: F, t673: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t682 = t190 * t581;
    let t684 = F::cast_from(4.0_f64) * t681 * t682;
    let t687 = piecewise3::<F>(t151, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t581);
    let t690 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t81 * t581);
    let t691 = t687 + t690;
    let t692 = t150 * t691;
    let t693 = t692 * t190;
    let t697 = t169 * t169;
    let t698 = F::cast_from(1.0_f64) / t697;
    let t699 = t164 * t698;
    let t704 = -F::cast_from(0.1176575e1_f64) * t662 - F::cast_from(0.516475e0_f64) * t664 - F::cast_from(0.2103875e0_f64) * t668 - F::cast_from(0.104195e0_f64) * t673;
    (t682, t684, t691, t692, t693, t697, t698, t699, t704)
}
