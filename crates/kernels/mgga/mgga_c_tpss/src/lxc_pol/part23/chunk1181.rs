//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1181/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1181<F: Float>(t19250: F, t19278: F, t3: F, t1281: F, t18584: F, t18586: F, t18588: F, t18591: F, t18595: F, t18598: F, t18601: F, t1904: F, t3407: F, t3410: F, t548: F, t6067: F) -> (F, F, F, F) {
    let t19279 = t19250 + t19278;
    let t19280 = t3 * t19279;
    let t19292 = param_d * t19279;
    let t19300 = 6.0 * t1281 * t6067 + 6.0 * t1904 * t3407 + 3.0 * t1904 * t3410 + t19292 * t548 + t18584 + t18586 + t18588 + t18591 + t18595 + t18598 + t18601;
    (t19279, t19280, t19292, t19300)
}
