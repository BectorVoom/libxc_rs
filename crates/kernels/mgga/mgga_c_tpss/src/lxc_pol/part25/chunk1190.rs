//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1190/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1190<F: Float>(t1791: F, t65157: F, t65165: F, t19342: F, t62348: F, t19349: F, t62342: F, t65208: F, t1675: F, t18645: F, t6090: F, t19352: F, t5791: F, t18660: F, t6073: F, t19411: F) -> (F, F, F, F, F, F, F, F, F) {
    let t67349 = t1791 * t65157;
    let t67352 = t1791 * t65165;
    let t67358 = 160.0 / 3.0 * t62348 * t19342;
    let t67369 = 160.0 / 9.0 * t19349 * t62342;
    let t67378 = t1791 * t65208;
    let t67385 = t1675 * t18645 * t6090;
    let t67389 = 16.0 / 9.0 * t19352 * t5791;
    let t67391 = 16.0 / 9.0 * t6073 * t18660;
    let t67429 = 32.0 / 9.0 * t19411 * t5791;
    (t67349, t67352, t67358, t67369, t67378, t67385, t67389, t67391, t67429)
}
