//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 664/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk664<F: Float>(t15015: F, t275: F, t15014: F, t290: F, t69995: F, t1347: F, t3286: F, t15017: F, t14935: F, t874: F, t70188: F, t70271: F, t70316: F, t69287: F, t3281: F, t4616: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t73395 = t275 * t15015;
    let t73397 = t290 * t15014;
    let t73411 = 0.17451485956252114153e-3 * t69995;
    let t73420 = t1347 * t3286;
    let t73448 = t275 * t15017;
    let t73450 = t874 * t14935;
    let t73454 = 0.46328831667894726561e-5 * t70188;
    let t73480 = 0.65053455985619242964e-5 * t70271;
    let t73484 = 0.65053455985619242964e-5 * t70316;
    let t73536 = 0.30643330512125015891e-2 * t69287;
    let t73569 = t4616 * t3281;
    (t73395, t73397, t73411, t73420, t73448, t73450, t73454, t73480, t73484, t73536, t73569)
}
