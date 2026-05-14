//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 471/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk471<F: Float>(t236: F, t458: F, t14125: F, t14124: F, t14121: F, t3128: F, t14115: F) -> (F, F, F, F) {
    let t14126 = t236 * t458;
    let t14127 = t14125 * t14126;
    let t14128 = t14124 * t14127;
    let t14130 = t3128 * t14121;
    let t14131 = t14130 * t14115;
    (t14127, t14128, t14130, t14131)
}
