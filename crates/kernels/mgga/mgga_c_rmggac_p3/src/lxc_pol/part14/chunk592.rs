//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 592/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk592<F: Float>(t2004: F, t7720: F, t2007: F, t1987: F, t1990: F, t333: F, t495: F, t511: F, t1971: F, t7230: F, t498: F, t7231: F) -> (F, F, F, F, F, F, F) {
    let t7721 = t7720 * t2004;
    let t7722 = F::cast_from(0.85129199786595678796e-5_f64) * t7721;
    let t7723 = t7720 * t2007;
    let t7724 = F::cast_from(0.25538759935978703638e-4_f64) * t7723;
    let t7725 = t7720 * t1987;
    let t7726 = F::cast_from(0.25538759935978703638e-4_f64) * t7725;
    let t7727 = t7720 * t1990;
    let t7728 = F::cast_from(0.85129199786595678796e-5_f64) * t7727;
    let t7731 = t333 * t495;
    let t7732 = t511 * t7731;
    let t7733 = t1971 * t7732;
    let t7734 = t7230 * t7733;
    let t7735 = F::cast_from(0.31923449919973379548e-4_f64) * t7734;
    let t7737 = t511 * t333 * t498;
    let t7738 = t7231 * t7737;
    (t7722, t7724, t7726, t7728, t7733, t7735, t7738)
}
