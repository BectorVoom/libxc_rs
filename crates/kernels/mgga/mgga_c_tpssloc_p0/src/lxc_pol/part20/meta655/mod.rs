//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2422;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta655<F: Float>(t10828: F, t300: F, t4475: F, t49514: F, t10753: F, t4488: F, t959: F, t14480: F, t2940: F, t2930: F, t1581: F, t13716: F, t2904: F, t952: F, t10623: F, t4498: F, t4493: F, t10629: F, t14259: F, t4471: F, t14260: F, t13663: F, t13718: F, t49082: F, t49084: F, t49086: F, t49088: F, t49090: F, t49092: F, t49095: F, t49228: F, t49244: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49535, t49538, t49540, t49544, t49548) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2422::<F>(t10828, t300, t4475, t49514, t10753, t4488, t959, t14480, t2940, t2930, t1581, t13716, t2904, t952);
        let (t49550, t49552, t49556, t49558, t49560, t49562, t49563) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2423::<F>(t10623, t4498, t4493, t10629, t14259, t4471, t959, t14260, t2940, t13663, t13718, t49082, t49084, t49086, t49088, t49090, t49092, t49095, t49228, t49244, t49535, t49538, t49540, t49544, t49548);
    (t49535, t49538, t49540, t49544, t49548, t49550, t49552, t49556, t49558, t49560, t49562, t49563)
}
