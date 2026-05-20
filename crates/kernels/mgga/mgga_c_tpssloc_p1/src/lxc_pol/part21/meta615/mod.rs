//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta615<F: Float>(t11998: F, t28: F, t517: F, t32253: F, t59: F, t154: F, t541: F, t12364: F, t3777: F, t1354: F, t12365: F, t3853: F) -> (F, F, F, F, F, F, F) {
        let (t39877, t39933, t39934, t39936, t39947, t39948, t39950) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2390::<F>(t11998, t28, t517, t32253, t59, t154, t541, t12364, t3777, t1354, t12365, t3853);
    (t39877, t39933, t39934, t39936, t39947, t39948, t39950)
}
