//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta578<F: Float>(t154: F, t21: F, t6896: F, t6898: F, t22797: F, t3770: F, t213: F, t6924: F, t9223: F, t6928: F, t22804: F, t22808: F) -> (F, F, F, F, F, F, F) {
        let (t80741, t80742, t80744, t80761, t80766, t80767, t80769) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1996::<F>(t154, t21, t6896, t6898, t22797, t3770, t213, t6924, t9223, t6928, t22804, t22808);
    (t80741, t80742, t80744, t80761, t80766, t80767, t80769)
}
