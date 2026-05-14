//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1280/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1280<F: Float>(t1317: F, t3482: F, t77: F, t1313: F, t3486: F, t21115: F, t619: F, t6076: F, t18335: F, t18342: F, t18345: F, t19404: F, t19408: F, t19425: F, t21123: F, t21129: F, t5487: F, t5503: F, t5507: F, t6077: F, t62033: F, t65413: F, t65417: F) -> (F,) {
    let t69135 = t77 * t3482 * t1317;
    let t69139 = t77 * t1313 * t3486;
    let t69143 = t77 * t21115 * t619;
    let t69147 = t77 * t6076 * t3486;
    let t69150 = 2.0 / 3.0 * t21123 * t5503 + 2.0 / 3.0 * t21123 * t5507 + 5.0 / 3.0 * t65413 * t6077 + 5.0 / 3.0 * t65417 * t6077 + 5.0 / 3.0 * t19425 * t19404 + 5.0 / 3.0 * t19425 * t19408 + 5.0 / 3.0 * t18335 * t21129 + 5.0 / 3.0 * t18342 * t21129 + 5.0 / 3.0 * t5487 * t69135 + 5.0 / 3.0 * t5487 * t69139 + 35.0 * t62033 * t69143 - 10.0 * t18345 * t69147;
    (t69150,)
}
