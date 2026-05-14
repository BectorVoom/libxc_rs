//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 288/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk288<F: Float>(t128: F, t797: F, t27: F, t321: F, t333: F, t352: F, t22: F, t29: F) -> (F, F, F, F, F) {
    let t2500 = t797 * t128;
    let t2518 = t27 * t321;
    let t2523 = t27 * t333;
    let t2529 = t27 * t352;
    let t2564 = t29 * t22;
    (t2500, t2518, t2523, t2529, t2564)
}
