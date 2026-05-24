//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 698/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk698<F: Float>(t511: F, t9969: F, t1971: F, t7230: F, t2144: F, t6557: F, t3351: F, t2289: F, t8571: F, t6349: F, t681: F, t2305: F, t8577: F) -> (F, F, F, F, F, F, F) {
    let t9970 = t511 * t9969;
    let t9971 = t1971 * t9970;
    let t9972 = t7230 * t9971;
    let t9973 = F::cast_from(0.31923449919973379548e-4_f64) * t9972;
    let t9974 = t2144 * t6557;
    let t9975 = t1971 * t9974;
    let t9976 = t3351 * t9975;
    let t9977 = F::cast_from(0.25538759935978703638e-4_f64) * t9976;
    let t9978 = t8571 * t2289;
    let t9979 = F::cast_from(0.25538759935978703638e-4_f64) * t9978;
    let t9980 = t6349 * t681;
    let t9981 = F::cast_from(0.14967802127329760705e-1_f64) * t9980;
    let t9982 = t8577 * t2305;
    (t9971, t9973, t9975, t9977, t9979, t9981, t9982)
}
