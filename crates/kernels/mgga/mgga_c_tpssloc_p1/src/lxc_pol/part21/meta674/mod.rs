//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta674<F: Float>(t111: F, t12723: F, t1454: F, t2585: F, t2281: F, t4044: F, t12758: F, t626: F, t12761: F, t12754: F, t4068: F, t12809: F) -> (F, F, F, F, F, F, F, F) {
        let (t45632, t45656, t45658, t45660, t45662, t45676, t45688, t45690) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2479::<F>(t111, t12723, t1454, t2585, t2281, t4044, t12758, t626, t12761, t12754, t4068, t12809);
    (t45632, t45656, t45658, t45660, t45662, t45676, t45688, t45690)
}
