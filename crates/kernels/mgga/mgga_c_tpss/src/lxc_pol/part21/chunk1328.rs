//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1328/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1328<F: Float>(t19424: F, t7690: F, t18325: F, t18328: F, t18332: F, t18335: F, t18347: F, t19396: F, t19404: F, t19408: F, t19417: F, t5503: F, t5507: F, t6077: F, t6080: F, t62039: F, t62042: F, t62047: F) -> (F,) {
    let t65258 = t7690 * t19424;
    let t65275 = 2.0 / 3.0 * t19417 * t5507 + t6080 * t18325 / 3.0 + 2.0 / 3.0 * t6080 * t18328 + t6080 * t18332 / 3.0 - 5.0 * t65258 * t18347 - 5.0 / 3.0 * t62039 * t6077 + 2.0 / 3.0 * t19396 * t5503 + 2.0 / 3.0 * t19396 * t5507 + 5.0 / 6.0 * t62042 * t6077 + 5.0 / 3.0 * t62047 * t6077 + 5.0 / 3.0 * t18335 * t19404 + 5.0 / 3.0 * t18335 * t19408;
    (t65275,)
}
