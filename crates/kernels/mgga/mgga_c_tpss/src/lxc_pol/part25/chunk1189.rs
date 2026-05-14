//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1189/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1189<F: Float>(t65616: F, t65628: F, t65639: F, t65643: F, t1844: F, t30367: F, t42181: F, t5784: F, t10292: F, t18669: F, t5489: F, t6077: F, t62280: F, t18670: F, t19404: F, t19408: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t67169 = 35.0 / 144.0 * t65616;
    let t67175 = 7.0 / 576.0 * t65628;
    let t67183 = 7.0 / 144.0 * t65639;
    let t67185 = 7.0 / 144.0 * t65643;
    let t67246 = t1844 * t30367;
    let t67326 = t42181 * t5784;
    let t67329 = t10292 * t18669;
    let t67331 = 80.0 / 9.0 * t67329 * t5489;
    let t67333 = 80.0 / 9.0 * t62280 * t6077;
    let t67335 = 80.0 / 9.0 * t18670 * t19404;
    let t67337 = 80.0 / 9.0 * t18670 * t19408;
    (t67169, t67175, t67183, t67185, t67246, t67326, t67329, t67331, t67333, t67335, t67337)
}
