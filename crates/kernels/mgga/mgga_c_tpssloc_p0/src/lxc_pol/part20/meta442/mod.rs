//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta442<F: Float>(t1147: F, t1156: F, t14829: F, t1164: F, t3423: F, t4869: F, t11126: F, t1703: F, t1657: F, t3263: F, t3266: F, t11292: F, t1694: F) -> (F, F, F, F, F, F, F) {
        let (t14831, t14833, t14835, t14837, t14838, t14840, t14841) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1886::<F>(t1147, t1156, t14829, t1164, t3423, t4869, t11126, t1703, t1657, t3263, t3266, t11292, t1694);
    (t14831, t14833, t14835, t14837, t14838, t14840, t14841)
}
