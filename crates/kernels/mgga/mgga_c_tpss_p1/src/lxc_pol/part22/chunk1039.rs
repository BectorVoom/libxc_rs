//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1039/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1039<F: Float>(t3770: F, t8737: F, t3811: F, t8595: F, t3807: F, t865: F, t2481: F, t1425: F, t2525: F, t2482: F, t3810: F, t8600: F) -> (F, F, F, F, F) {
    let t11240 = F::new(4.0) * t8737 * t3770;
    let t11242 = F::cast_from(0.32163958997385070134e2_f64) * t8595 * t3811;
    let t11243 = t3807 * t865;
    let t11245 = F::new(4.0) * t2481 * t11243;
    let t11246 = t1425 * t2525;
    let t11248 = F::new(2.0) * t2481 * t11246;
    let t11249 = t3810 * t2482;
    let t11251 = F::cast_from(0.96491876992155210402e2_f64) * t8600 * t11249;
    (t11240, t11242, t11245, t11248, t11251)
}
