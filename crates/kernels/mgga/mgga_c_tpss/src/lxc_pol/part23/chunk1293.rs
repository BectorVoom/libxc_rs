//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1293/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1293<F: Float>(t10416: F, t578: F, t10425: F, t1317: F, t2045: F, t77: F, t3486: F, t615: F, t10440: F, t84: F, t1290: F, t7679: F, t1976: F, t3426: F, t3432: F, t3482: F, t619: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65237 = t578 * t10416;
    let t65244 = t578 * t10425;
    let t65285 = t77 * t2045 * t1317;
    let t65289 = t77 * t615 * t3486;
    let t65293 = t77 * t84 * t10440;
    let t65296 = t7679 * t1290;
    let t65299 = t1976 * t3426;
    let t65302 = t1976 * t3432;
    let t65321 = t77 * t3482 * t619;
    (t65237, t65244, t65285, t65289, t65293, t65296, t65299, t65302, t65321)
}
