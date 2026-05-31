//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 617/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk617<F: Float>(t219: F, t3641: F, t3648: F, t222: F, t73: F, t1364: F, t799: F, t750: F, t3610: F, t778: F, t1373: F, t1375: F, t224: F, t776: F, t779: F) -> (F, F, F, F, F, F) {
    let t3650 = (t3641 + t3648) * t219;
    let t3656 = t222 * t73;
    let t3657 = t799 * t1364;
    let t3658 = t3657 * t750;
    let t3661 = t778 * t3610;
    let t3664 = F::cast_from(3.0_f64) * t1373 * t779 + F::cast_from(3.0_f64) * t1375 * t776 + F::cast_from(3.0_f64) * t222 * t3661 - t224 * t3650 - F::cast_from(12.0_f64) * t3656 * t3658;
    (t3650, t3656, t3657, t3658, t3661, t3664)
}
