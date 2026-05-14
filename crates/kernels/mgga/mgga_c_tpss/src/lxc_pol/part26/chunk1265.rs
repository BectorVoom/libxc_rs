//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1265/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1265<F: Float>(t1290: F, t7682: F, t1981: F, t3426: F, t3432: F, t10292: F, t582: F, t6090: F, t619: F, t1333: F, t61870: F, t19590: F, t61873: F, t18394: F, t3532: F, t18546: F, t6242: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65169 = t7682 * t1290;
    let t65172 = t1981 * t3426;
    let t65175 = t1981 * t3432;
    let t65189 = t10292 * t582;
    let t65208 = t6090 * t619;
    let t65440 = t61870 * t1333;
    let t65442 = t61873 * t19590;
    let t65443 = 4.0 / 3.0 * t65442;
    let t65444 = t18394 * t3532;
    let t65445 = 2.0 / 3.0 * t65444;
    let t65533 = t6242 * t18546;
    (t65169, t65172, t65175, t65189, t65208, t65440, t65443, t65445, t65533)
}
