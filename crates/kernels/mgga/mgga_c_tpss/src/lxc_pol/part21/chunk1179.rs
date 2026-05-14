//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1179/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1179<F: Float>(t2045: F, t76: F, t1678: F, t5486: F, t7682: F, t1976: F, t582: F, t38: F, t5501: F, t1981: F, t7690: F) -> (F, F, F, F, F, F, F) {
    let t18331 = t76 * t2045;
    let t18332 = t1678 * t18331;
    let t18335 = t7682 * t5486;
    let t18338 = t1976 * t582;
    let t18341 = t38 * t5501;
    let t18342 = t1981 * t18341;
    let t18345 = t7690 * t5486;
    (t18331, t18332, t18335, t18338, t18341, t18342, t18345)
}
