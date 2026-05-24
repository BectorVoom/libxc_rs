//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 830/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk830<F: Float>(t36629: F, t38565: F, t352: F, t8712: F, t262: F, t7192: F, t16043: F, t9190: F, t9194: F, t9198: F, t2286: F, t35277: F) -> (F, F, F, F, F, F, F, F) {
    let t38566 = t36629 * t38565;
    let t38568 = t8712 * t352;
    let t38569 = t262 * t38568;
    let t38570 = t7192 * t38569;
    let t38572 = t16043 * t9190;
    let t38574 = t16043 * t9194;
    let t38576 = t16043 * t9198;
    let t38578 = t35277 * t2286;
    (t38566, t38568, t38569, t38570, t38572, t38574, t38576, t38578)
}
