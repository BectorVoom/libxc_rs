//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 827/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk827<F: Float>(t191: F, t192: F, t23855: F, t22947: F, t3701: F, t31054: F, t31056: F, t31059: F, t214: F, t6624: F, t1880: F, t6572: F, t23218: F, t30663: F, t30657: F, t6547: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112547 = t23855 * t191 * t192;
    let t112611 = t3701 * t22947;
    let t112620 = 4.0 * t31054;
    let t112621 = 4.0 * t31056;
    let t112622 = 4.0 * t31059;
    let t112660 = t214 * t6624;
    let t112663 = 0.3289868133696452873e-1 * t1880 * t112660 * t6572;
    let t112666 = 0.16449340668482264365e-1 * t1880 * t30663 * t23218;
    let t112667 = t6547 * t30657;
    (t112547, t112611, t112620, t112621, t112622, t112660, t112663, t112666, t112667)
}
