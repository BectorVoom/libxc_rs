//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1329/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1329<F: Float>(t1317: F, t2045: F, t77: F, t3486: F, t615: F, t10440: F, t84: F, t1290: F, t7679: F, t1976: F, t3426: F, t3432: F, t1680: F, t18338: F, t18342: F, t18366: F, t19404: F, t19408: F, t19411: F, t5487: F, t5503: F, t6077: F, t6087: F, t62060: F) -> (F,) {
    let t65285 = t77 * t2045 * t1317;
    let t65289 = t77 * t615 * t3486;
    let t65293 = t77 * t84 * t10440;
    let t65296 = t7679 * t1290;
    let t65299 = t1976 * t3426;
    let t65302 = t1976 * t3432;
    let t65311 = 5.0 / 6.0 * t62060 * t6077 + 5.0 / 3.0 * t18342 * t19404 + 5.0 / 3.0 * t18342 * t19408 + 5.0 / 6.0 * t5487 * t65285 + 5.0 / 3.0 * t5487 * t65289 + 5.0 / 6.0 * t5487 * t65293 + t65296 * t1680 / 3.0 + 2.0 / 3.0 * t65299 * t1680 + 2.0 / 3.0 * t65302 * t1680 + 2.0 / 3.0 * t19411 * t5503 + 2.0 / 3.0 * t18338 * t6087 + t18366 * t6087 / 3.0;
    (t65311,)
}
