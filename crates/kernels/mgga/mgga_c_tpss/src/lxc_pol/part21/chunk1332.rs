//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1332/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1332<F: Float>(t10408: F, t76: F, t10289: F, t582: F, t1993: F, t3418: F, t42181: F, t5486: F, t1313: F, t1982: F, t77: F, t19424: F, t7682: F, t19367: F, t1981: F, t38: F) -> (F, F, F, F, F, F, F) {
    let t65396 = t76 * t10408;
    let t65400 = t10289 * t582;
    let t65403 = t3418 * t1993;
    let t65406 = t42181 * t5486;
    let t65410 = t77 * t1313 * t1982;
    let t65413 = t7682 * t19424;
    let t65417 = t1981 * t38 * t19367;
    (t65396, t65400, t65403, t65406, t65410, t65413, t65417)
}
