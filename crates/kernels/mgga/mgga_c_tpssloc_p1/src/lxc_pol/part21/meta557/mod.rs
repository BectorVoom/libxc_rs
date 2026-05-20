//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2260;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta557<F: Float>(t18630: F, t18673: F, t18789: F, t18906: F, t300: F, t3400: F, t6084: F, t4883: F, t1164: F, t18247: F, t18249: F, t18251: F, t18257: F, t18261: F, t18264: F, t18268: F, t18270: F, t18273: F, t18278: F, t18282: F, t18285: F, t18672: F, t18676: F, t18679: F, t6063: F, t1166: F, t4858: F, t4874: F, t3411: F, t6098: F, t4869: F, t4884: F, t1147: F, t1156: F, t18785: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18909, t18910, t18911, t18913, t18914) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2260::<F>(t18630, t18673, t18789, t18906, t300, t3400, t6084, t4883, t1164, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18672, t18676, t18679);
        let (t18915, t18917, t18918, t18920, t18922, t18924, t18926) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2261::<F>(t300, t6063, t1166, t4858, t4874, t1164, t3411, t6098, t4869, t4884, t1147, t1156, t18785);
    (t18909, t18910, t18911, t18913, t18914, t18915, t18917, t18918, t18920, t18922, t18924, t18926)
}
