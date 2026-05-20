//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta638<F: Float>(t136: F, t2826: F, t47726: F, t13560: F, t699: F, t47759: F, t908: F, t47689: F, t2403: F, t4392: F, t13646: F, t47734: F) -> (F, F, F, F, F, F, F, F) {
        let (t48085, t48087, t48090, t48092, t48096, t48097, t48098, t48101) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2345::<F>(t136, t2826, t47726, t13560, t699, t47759, t908, t47689, t2403, t4392, t13646, t47734);
    (t48085, t48087, t48090, t48092, t48096, t48097, t48098, t48101)
}
