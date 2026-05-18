//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 857/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk857<F: Float>(t69279: F, t75346: F, t75351: F, t25525: F, t3068: F, t75355: F, t2044: F, t25640: F, t75359: F, t69276: F, t75318: F, t75321: F) -> (F, F, F, F, F, F) {
    let t75378 = t69279 * t75346;
    let t75380 = t69279 * t75351;
    let t75383 = t25525 * t3068 * t75355;
    let t75386 = t25640 * t2044 * t75359;
    let t75388 = t69276 * t75318;
    let t75390 = t69276 * t75321;
    (t75378, t75380, t75383, t75386, t75388, t75390)
}
