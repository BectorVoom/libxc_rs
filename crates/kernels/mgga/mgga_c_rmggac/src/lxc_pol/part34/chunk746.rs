//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 746/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk746<F: Float>(t71660: F, t70071: F, t70078: F, t70082: F, t14494: F, t874: F, t14563: F, t2160: F, t638: F, t14559: F, t70188: F, t70237: F) -> (F, F, F, F, F, F, F, F, F) {
    let t71661 = F::cast_from(0.34200192530023447503e-6_f64) * t71660;
    let t71670 = F::cast_from(0.66671395154821946452e-1_f64) * t70071;
    let t71671 = F::cast_from(0.39032073591371545778e-3_f64) * t70078;
    let t71672 = F::cast_from(0.30487649791575028312e-3_f64) * t70082;
    let t71704 = t874 * t14494;
    let t71717 = t638 * t2160 * t14563;
    let t71720 = t638 * t2160 * t14559;
    let t71727 = F::cast_from(0.46328831667894726564e-5_f64) * t70188;
    let t71744 = F::cast_from(0.60975299583150056624e-3_f64) * t70237;
    (t71661, t71670, t71671, t71672, t71704, t71717, t71720, t71727, t71744)
}
