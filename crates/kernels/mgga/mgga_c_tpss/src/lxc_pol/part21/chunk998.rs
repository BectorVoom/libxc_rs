//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 998/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk998<F: Float>(t10621: F, t10656: F, t10772: F, t10816: F, t219: F, t3693: F, t1395: F, t2407: F, t8348: F, t3721: F, t818: F, t2406: F, t2425: F, t220: F, t73: F, t8275: F) -> (F, F, F, F, F, F, F) {
    let t10818 = t10621 + t10656 + t10772 + t10816;
    let t10819 = param_beta * t10818;
    let t10821 = t3693 * t219;
    let t10833 = t8348 * t1395 * t2407;
    let t10836 = t3721 * t818;
    let t10837 = t2406 * t10836;
    let t10841 = t2406 * t1395 * t2425;
    let t10845 = t220 * t73 * t8275;
    (t10818, t10819, t10821, t10833, t10837, t10841, t10845)
}
