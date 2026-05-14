//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 726/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk726<F: Float>(t69574: F, t15382: F, t321: F, t3352: F, t515: F, t7230: F, t15252: F, t3351: F, t352: F, t7231: F, t875: F, t118: F, t2001: F, t618: F, t665: F, t7720: F) -> (F, F, F, F) {
    let t75110 = 0.23948483403727617128e0 * t69574;
    let t75115 = 0.3192344991997337955e-4 * t7230 * t3352 * t515 * t15382 * t321;
    let t75119 = t3351 * t7231 * t875 * t15252 * t352;
    let t75123 = t2001 * t118 * t665 * t618;
    let t75124 = t7720 * t75123;
    (t75110, t75115, t75119, t75124)
}
