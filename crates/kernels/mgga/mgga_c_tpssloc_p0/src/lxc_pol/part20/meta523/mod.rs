//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta523<F: Float>(t1354: F, t39947: F, t12365: F, t3853: F, t12267: F, t3798: F, t12297: F, t12385: F, t12300: F, t3858: F, t12283: F, t12404: F) -> (F, F, F, F, F, F, F) {
        let (t39948, t39950, t39955, t39956, t39958, t39960, t39971) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2057::<F>(t1354, t39947, t12365, t3853, t12267, t3798, t12297, t12385, t12300, t3858, t12283, t12404);
    (t39948, t39950, t39955, t39956, t39958, t39960, t39971)
}
