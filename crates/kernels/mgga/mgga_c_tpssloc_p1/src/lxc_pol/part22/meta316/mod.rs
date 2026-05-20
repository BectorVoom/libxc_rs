//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1496;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1497;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta316<F: Float>(t1227: F, t15486: F, t3536: F, t4997: F, t248: F, t3570: F, t5012: F, t1213: F, t3535: F, t5018: F, t1202: F, t5023: F, t1742: F, t3036: F, t3503: F, t3500: F, t1210: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15488, t15490, t15492, t15494, t15495, t15498) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1496::<F>(t1227, t15486, t3536, t4997, t248, t3570, t5012, t1213, t3535, t5018, t1202, t5023);
        let (t15501, t15502, t15503) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1497::<F>(t1742, t3036, t3503, t3500);
        let (t15506, t15507) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1498::<F>(t1210, t15501, t3500);
    (t15488, t15490, t15492, t15494, t15495, t15498, t15502, t15503, t15506, t15507)
}
