//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 470/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk470<F: Float>(t60: F, t1805: F, t921: F, t5860: F, t1403: F, t284: F, t5865: F, t62: F, t815: F, t5864: F, t277: F, t352: F, t570: F, zeta_threshold: F) -> (F, F, F, F) {
    let t61 = t60 <= zeta_threshold;
    let t5870 = t921 * t1805;
    let t5873 = -t5860;
    let t5877 = piecewise3::<F>(t61, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5865 * t284 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1403 * t815 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5870 * t284 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t62 * t5873);
    let t5878 = t5864 + t5877;
    let t5879 = t277 * t5878;
    let t5888 = t570 * t352;
    (t5873, t5878, t5879, t5888)
}
