//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1327/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1327<F: Float>(t19403: F, t619: F, t77: F, t6090: F, t1985: F, t3418: F, t38: F, t41937: F, t10412: F, t578: F, t10416: F, t10425: F) -> (F, F, F, F, F, F, F) {
    let t65202 = t77 * t19403 * t619;
    let t65208 = t6090 * t619;
    let t65214 = t3418 * t1985;
    let t65217 = t41937 * t38;
    let t65234 = t578 * t10412;
    let t65237 = t578 * t10416;
    let t65244 = t578 * t10425;
    (t65202, t65208, t65214, t65217, t65234, t65237, t65244)
}
