//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 932/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk932<F: Float>(t2298: F, t24363: F, t1614: F, t3351: F, t498: F, t511: F, t7231: F, t34724: F, t8626: F, t504: F, t8629: F, t8632: F) -> (F, F, F, F) {
    let t40050 = t24363 * t2298;
    let t40055 = t3351 * t7231 * t511 * t1614 * t498;
    let t40057 = t34724 * t8626;
    let t40060 = t504 * t8629 * t8632;
    (t40050, t40055, t40057, t40060)
}
