//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1244/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1244<F: Float>(t2061: F, t6308: F, t42181: F, t5784: F, t10292: F, t18669: F, t5489: F, t6077: F, t62280: F, t18670: F, t19404: F, t19408: F, t18347: F, t18649: F, t5785: F, t62247: F, t62250: F, t62311: F, t62330: F, t65285: F, t65293: F) -> (F, F) {
    let t67316 = t6308 * t2061;
    let t67326 = t42181 * t5784;
    let t67329 = t10292 * t18669;
    let t67331 = 80.0 / 9.0 * t67329 * t5489;
    let t67333 = 80.0 / 9.0 * t62280 * t6077;
    let t67335 = 80.0 / 9.0 * t18670 * t19404;
    let t67337 = 80.0 / 9.0 * t18670 * t19408;
    let t67342 = -5.0 / 3.0 * t62311 * t6077 - 10.0 / 3.0 * t18649 * t19404 + 10.0 / 3.0 * t62330 * t6077 - 5.0 / 3.0 * t5785 * t65285 + 10.0 * t67326 * t18347 + t67331 + t67333 + t67335 + t67337 - 5.0 / 3.0 * t5785 * t65293 + 16.0 / 9.0 * t62247 - 8.0 / 9.0 * t62250;
    (t67316, t67342)
}
