//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 624/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk624<F: Float>(t15597: F, t338: F, t118: F, t14354: F, t15175: F, t15421: F, t15439: F, t15442: F, t15571: F, t15573: F, t15574: F, t15581: F, t15584: F, t305: F, t326: F) -> (F, F) {
    let t15598 = t338 * t15597;
    let t15599 = t118 * t15598;
    let t15603 = -t15571 - t15573 - t15175 + t15574 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t15439 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t15442 - t15581 + t15584 + t14354 + F::cast_from(0.19957069503106347607e-1_f64) * t15599 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t15421;
    (t15598, t15603)
}
