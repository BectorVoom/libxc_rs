//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1106/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1106<F: Float>(t3030: F, t344: F, t1014: F, t1011: F, t360: F, t3187: F, t3192: F, t6800: F, t6799: F, t225: F, t6733: F, t6786: F, t1949: F, t2966: F, t1920: F, t1948: F, t3166: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23602 = t344 * t3030;
    let t23603 = t23602 * t1014;
    let t23604 = t1011 * t360;
    let t23605 = t3187 * t23604;
    let t23606 = t23603 * t23605;
    let t23609 = t3192 * t6800;
    let t23610 = t6799 * t23609;
    let t23613 = t6733 * t225;
    let t23614 = t23613 * t6786;
    let t23617 = t2966 * t1949;
    let t23619 = 0.18277045187202515961e-2 * t1920 * t23617;
    let t23620 = t1948 * t3166;
    (t23602, t23603, t23604, t23605, t23606, t23609, t23610, t23613, t23614, t23617, t23619, t23620)
}
