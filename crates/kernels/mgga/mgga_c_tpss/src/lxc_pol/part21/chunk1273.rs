//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1273/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1273<F: Float>(t31455: F, t5486: F, t18341: F, t7690: F, t31464: F, t1677: F, t1981: F, t1985: F, t31450: F, t7682: F, t18323: F, t38: F, t18602: F, t546: F, t116: F, t18403: F) -> (F, F, F, F, F, F, F, F, F) {
    let t62027 = t31455 * t5486;
    let t62030 = t7690 * t18341;
    let t62033 = t31464 * t5486;
    let t62039 = t1981 * t1985 * t1677;
    let t62042 = t31450 * t5486;
    let t62047 = t7682 * t18341;
    let t62060 = t1981 * t38 * t18323;
    let t62104 = t546 * t18602;
    let t62124 = t116 * t18403;
    (t62027, t62030, t62033, t62039, t62042, t62047, t62060, t62104, t62124)
}
