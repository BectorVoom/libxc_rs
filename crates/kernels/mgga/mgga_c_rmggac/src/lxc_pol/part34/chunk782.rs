//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 782/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk782<F: Float>(t15252: F, t3351: F, t498: F, t515: F, t9210: F, t3928: F, t76270: F, t70545: F, t14102: F, t8365: F, t638: F, t639: F, t640: F, t9030: F, t2046: F, t3047: F, t8850: F) -> (F, F, F, F, F, F) {
    let t76504 = t3351 * t9210 * t515 * t15252 * t498;
    let t76506 = t3928 * t76270;
    let t76515 = 0.79828278012425390427e-1 * t70545;
    let t76517 = t8365 * t14102;
    let t76521 = t638 * t639 * t640 * t9030;
    let t76524 = t2046 * t3047 * t8850;
    (t76504, t76506, t76515, t76517, t76521, t76524)
}
