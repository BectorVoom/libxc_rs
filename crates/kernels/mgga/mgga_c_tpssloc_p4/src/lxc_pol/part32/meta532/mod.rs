//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta532<F: Float>(t24601: F, t27437: F, t24590: F, t8002: F, t3247: F, t497: F, t3961: F, t24574: F, t8067: F, t1184: F, t1715: F, t24745: F, t7363: F) -> (F, F, F, F, F, F, F, F) {
        let (t27438, t27441, t27444, t27445, t27446, t27451, t27453, t27454) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1869::<F>(t24601, t27437, t24590, t8002, t3247, t497, t3961, t24574, t8067, t1184, t1715, t24745, t7363);
    (t27438, t27441, t27444, t27445, t27446, t27451, t27453, t27454)
}
