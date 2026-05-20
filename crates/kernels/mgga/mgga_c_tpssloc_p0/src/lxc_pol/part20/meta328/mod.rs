//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1611;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1612;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta328<F: Float>(t11720: F, t3508: F, t1214: F, t248: F, t11708: F, t3514: F, t11717: F, t1210: F, t11713: F, t475: F, t3509: F, t3570: F, t3506: F, t11159: F, t3440: F, t11168: F, t1177: F, t135: F, t3561: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11729, t11731, t11734) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1611::<F>(t11720, t3508, t1214, t248, t11708, t3514);
        let (t11737, t11738) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1612::<F>(t11717, t1210, t11713);
        let (t11739, t11741, t11745, t11746, t11748, t11751, t11754) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1613::<F>(t11720, t475, t1214, t248, t3509, t3570, t3506, t11159, t3440, t11168, t1177, t135, t3561);
    (t11729, t11731, t11734, t11737, t11738, t11739, t11741, t11745, t11746, t11748, t11751, t11754)
}
