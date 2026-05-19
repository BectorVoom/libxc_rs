//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 803/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk803<F: Float>(t69971: F, t9222: F, t27: F, t8455: F, t16129: F, t70489: F, t201: F, t209: F, t457: F, t68504: F, t68505: F, t8440: F) -> (F, F, F) {
    let t74468 = F::cast_from(0.1064114997332445985e-4_f64) * t9222 * t69971;
    let t74469 = t27 * t8455;
    let t74471 = t70489 * t16129 * t74469;
    let t74477 = t68504 * t68505 * t8440 * t209 * t457 * t201;
    (t74468, t74471, t74477)
}
