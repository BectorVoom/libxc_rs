//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 551/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk551<F: Float>(t14434: F, t352: F, t2228: F, t36: F, t305: F, t664: F, t8264: F, t118: F, t698: F) -> (F, F, F, F, F, F) {
    let t14435 = t14434 * t352;
    let t14438 = t2228 * t36;
    let t14439 = t305 * t14438;
    let t14440 = F::new(0.14967802127329760705e-1) * t14439;
    let t14441 = t8264 * t664;
    let t14443 = F::new(0.39914139006212695214e-1) * t118 * t14441;
    let t14444 = t698 * t664;
    (t14435, t14438, t14440, t14441, t14443, t14444)
}
