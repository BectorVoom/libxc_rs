//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1154/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1154<F: Float>(t10364: F, t10365: F, t10366: F, t10369: F, t37179: F, t42520: F, t42521: F, t42527: F, t7762: F, t8192: F, t8193: F, t10377: F, t10378: F, t10380: F, t37183: F, t42535: F, t42536: F, t42537: F, t42539: F, t42540: F, t8197: F, t9499: F) -> (F, F) {
    let t49867 = t10364 + t10365 - t10366 - t42520 + t10369 + t42521 + t8192 + t8193 - F::cast_from(0.20496175532535769483e-3_f64) * t7762 + t37179 - t42527;
    let t49872 = t42535 + t42536 + t42537 + F::cast_from(4.0_f64) * t9499 - t8197 + t10377 - t10378 + t42539 + t42540 + t37183 + t10380;
    (t49867, t49872)
}
