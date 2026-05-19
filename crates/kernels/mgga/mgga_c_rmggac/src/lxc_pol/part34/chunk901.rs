//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 901/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk901<F: Float>(t4669: F, t75200: F, t25820: F, t75956: F, t27101: F, t75962: F, t41407: F, t649: F, t8982: F, t40928: F, t8963: F, t40932: F, t8937: F) -> (F, F, F, F, F, F) {
    let t76140 = t4669 * t75200;
    let t76141 = F::cast_from(0.23948483403727617128e0_f64) * t76140;
    let t76143 = t25820 * t75956;
    let t76145 = t27101 * t75962;
    let t76148 = t41407 * t649 * t8982;
    let t76151 = t40928 * t649 * t8963;
    let t76154 = t40932 * t649 * t8937;
    (t76141, t76143, t76145, t76148, t76151, t76154)
}
