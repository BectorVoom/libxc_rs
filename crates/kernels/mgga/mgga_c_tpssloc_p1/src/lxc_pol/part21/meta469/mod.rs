//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2046;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta469<F: Float>(t1343: F, t16206: F, t820: F, t12365: F, t1827: F, t12300: F, t1799: F, t3734: F, t12351: F, t12418: F, t1351: F, t3807: F, t12289: F, t242: F, t1336: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16208, t16211, t16214, t16215, t16217, t16224) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2046::<F>(t1343, t16206, t820, t12365, t1827, t12300, t1799, t3734, t12351, t12418);
        let (t16225, t16226, t16227, t16232, t16233) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2047::<F>(t1351, t1799, t3807, t16224, t12289, t242, t1336);
    (t16208, t16211, t16214, t16215, t16217, t16224, t16225, t16226, t16227, t16232, t16233)
}
