//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1294/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1294<F: Float>(t1313: F, t2049: F, t77: F, t10408: F, t76: F, t10289: F, t582: F, t1993: F, t3418: F, t1982: F, t12841: F, t19620: F, t7310: F, t61871: F, t1333: F, t61870: F) -> (F, F, F, F, F, F, F, F) {
    let t65325 = t77 * t1313 * t2049;
    let t65396 = t76 * t10408;
    let t65400 = t10289 * t582;
    let t65403 = t3418 * t1993;
    let t65410 = t77 * t1313 * t1982;
    let t65436 = 6.0 * t19620 * t7310 * t12841;
    let t65437 = 22.0 / 9.0 * t61871;
    let t65440 = t61870 * t1333;
    (t65325, t65396, t65400, t65403, t65410, t65436, t65437, t65440)
}
