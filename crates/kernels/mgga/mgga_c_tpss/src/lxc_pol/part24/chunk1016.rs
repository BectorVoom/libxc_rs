//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1016/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1016<F: Float>(t14261: F, t14262: F, t14264: F, t14265: F, t219: F, t1373: F, t73: F, t2387: F, t4706: F, t750: F, t3610: F, t3657: F, t4701: F, t799: F, t14029: F, t778: F) -> (F, F, F, F, F, F) {
    let t14268 = (t14261 + t14262 + t14264 + t14265) * t219;
    let t14274 = t1373 * t73;
    let t14281 = t2387 * t4706;
    let t14282 = t14281 * t750;
    let t14285 = t3657 * t3610;
    let t14290 = t799 * t4701;
    let t14291 = t14290 * t750;
    let t14294 = t778 * t14029;
    (t14268, t14274, t14282, t14285, t14291, t14294)
}
