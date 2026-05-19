//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 939/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk939<F: Float>(t76727: F, t15502: F, t2144: F, t333: F, t3351: F, t7231: F, t352: F, t875: F, t118: F, t2001: F, t618: F, t699: F) -> (F, F, F, F) {
    let t76728 = F::cast_from(0.12769379967989351819e-4_f64) * t76727;
    let t76732 = t3351 * t7231 * t2144 * t15502 * t333;
    let t76733 = F::cast_from(0.12769379967989351819e-4_f64) * t76732;
    let t76737 = t3351 * t7231 * t875 * t15502 * t352;
    let t76738 = F::cast_from(0.85129199786595678796e-5_f64) * t76737;
    let t76741 = t2001 * t118 * t699 * t618;
    (t76728, t76733, t76738, t76741)
}
