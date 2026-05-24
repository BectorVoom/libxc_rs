//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1046/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1046<F: Float>(t10088: F, t321: F, t3351: F, t511: F, t7248: F, t333: F, t7231: F, t880: F, t9128: F, t9765: F, t2186: F, t9817: F) -> (F, F, F, F) {
    let t47202 = t3351 * t7248 * t511 * t10088 * t321;
    let t47207 = t3351 * t7231 * t880 * t10088 * t333;
    let t47213 = t9128 * t9765;
    let t47215 = t2186 * t9817;
    (t47202, t47207, t47213, t47215)
}
