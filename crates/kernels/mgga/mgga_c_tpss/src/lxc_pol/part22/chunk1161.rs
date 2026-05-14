//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1161/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1161<F: Float>(t1395: F, t5831: F, t5572: F, t1805: F, t3721: F, t18770: F, t19762: F, t2157: F, t19769: F, t1378: F, t226: F, t5577: F, t3664: F, t19781: F, t6337: F, t818: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20470 = t5831 * t1395;
    let t20471 = t5572 * t20470;
    let t20474 = t1805 * t3721;
    let t20475 = t5572 * t20474;
    let t20479 = t18770 * t19762;
    let t20482 = t2157 * t1805;
    let t20483 = t20482 * t19769;
    let t20487 = t5831 * t1378 * t226;
    let t20488 = t5577 * t20487;
    let t20492 = t5577 * t1805 * t3664 * t226;
    let t20494 = t18770 * t19781;
    let t20498 = t5572 * t6337 * t818;
    (t20470, t20471, t20474, t20475, t20479, t20482, t20483, t20488, t20492, t20494, t20498)
}
