//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1292/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1292<F: Float>(t1290: F, t7690: F, t1982: F, t6076: F, t77: F, t10292: F, t582: F, t19403: F, t619: F, t6090: F, t1985: F, t3418: F, t38: F, t41937: F, t10412: F, t578: F) -> (F, F, F, F, F, F, F, F) {
    let t65178 = t7690 * t1290;
    let t65182 = t77 * t6076 * t1982;
    let t65189 = t10292 * t582;
    let t65202 = t77 * t19403 * t619;
    let t65208 = t6090 * t619;
    let t65214 = t3418 * t1985;
    let t65217 = t41937 * t38;
    let t65234 = t578 * t10412;
    (t65178, t65182, t65189, t65202, t65208, t65214, t65217, t65234)
}
