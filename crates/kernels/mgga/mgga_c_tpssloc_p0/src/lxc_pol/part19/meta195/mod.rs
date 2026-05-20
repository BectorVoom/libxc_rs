//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta195<F: Float>(t10216: F, t344: F, t9288: F, t10214: F, t698: F, t976: F, t979: F, t973: F, t2970: F, t2999: F, t135: F, t2978: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10217, t10218, t10219, t10224, t10225, t10226, t10228, t10229, t10231) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk860::<F>(t10216, t344, t9288, t10214, t698, t976, t979, t973, t2970, t2999, t135, t2978);
    (t10217, t10218, t10219, t10224, t10225, t10226, t10228, t10229, t10231)
}
