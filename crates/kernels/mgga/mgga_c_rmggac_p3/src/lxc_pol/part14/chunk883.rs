//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 883/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk883<F: Float>(t39277: F, t7239: F, t7733: F, t275: F, t8869: F, t2405: F, t934: F, t16043: F, t9111: F, t2283: F, t35277: F, t1528: F, t236: F, t495: F, t7230: F, t7231: F) -> (F, F, F, F, F, F, F) {
    let t39314 = t39277 * t7239;
    let t39316 = t39277 * t7733;
    let t39319 = F::cast_from(2.0_f64) * t275 * t8869;
    let t39320 = t934 * t2405;
    let t39323 = t16043 * t9111;
    let t39325 = t35277 * t2283;
    let t39330 = t7230 * t7231 * t236 * t1528 * t495;
    (t39314, t39316, t39319, t39320, t39323, t39325, t39330)
}
