//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1850/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1850<F: Float>(t1014: F, t23602: F, t1011: F, t360: F, t3187: F, t3192: F, t6800: F, t6799: F, t225: F, t6733: F) -> (F, F, F, F, F, F, F) {
    let t23603 = t23602 * t1014;
    let t23604 = t1011 * t360;
    let t23605 = t3187 * t23604;
    let t23606 = t23603 * t23605;
    let t23609 = t3192 * t6800;
    let t23610 = t6799 * t23609;
    let t23613 = t6733 * t225;
    (t23603, t23604, t23605, t23606, t23609, t23610, t23613)
}
