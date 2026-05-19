//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 627/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk627<F: Float>(t15881: F, t338: F, t118: F, t14354: F, t15175: F, t15564: F, t15566: F, t15568: F, t15571: F, t15573: F, t15574: F, t15581: F, t15584: F) -> (F, F) {
    let t15882 = t338 * t15881;
    let t15883 = t118 * t15882;
    let t15885 = t15564 - t15566 - t15568 - t15571 - t15573 - t15175 + t15574 - t15581 + t15584 + t14354 + F::cast_from(0.19957069503106347607e-1_f64) * t15883;
    (t15882, t15885)
}
