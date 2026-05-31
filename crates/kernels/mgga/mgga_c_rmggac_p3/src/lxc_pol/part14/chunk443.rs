//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 443/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk443<F: Float>(t50: F, t75: F, t80: F, t1279: F, t299: F, t1285: F, t295: F, t1328: F, t78: F, t76: F, t1296: F, t252: F) -> (F, F, F, F, F) {
    let t4695 = t75 * t50;
    let t4697 = F::cast_from(1320.0_f64) * t4695 * t80;
    let t4698 = t1279 * t299;
    let t4700 = t295 * t1285;
    let t4703 = F::cast_from(1.0_f64) / t78 / t1328;
    let t4705 = F::cast_from(2184.0_f64) * t76 * t4703;
    let t4709 = t1296 * t252;
    (t4697, t4698, t4700, t4705, t4709)
}
