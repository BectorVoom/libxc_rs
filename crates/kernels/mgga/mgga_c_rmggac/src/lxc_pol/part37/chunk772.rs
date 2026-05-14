//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 772/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk772<F: Float>(t69626: F, t8571: F, t15361: F, t352: F, t14236: F, t14237: F, t2078: F, t4669: F, t75200: F, t25820: F, t75956: F, t27101: F, t75962: F, t41407: F, t649: F, t8982: F) -> (F, F, F, F, F, F) {
    let t76132 = t8571 * t69626;
    let t76134 = t15361 * t352;
    let t76137 = t14236 * t14237 * t2078 * t76134;
    let t76140 = t4669 * t75200;
    let t76141 = 0.23948483403727617128e0 * t76140;
    let t76143 = t25820 * t75956;
    let t76145 = t27101 * t75962;
    let t76148 = t41407 * t649 * t8982;
    (t76132, t76137, t76141, t76143, t76145, t76148)
}
