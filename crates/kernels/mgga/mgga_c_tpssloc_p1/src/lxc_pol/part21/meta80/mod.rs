//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta80 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk578;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk579;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk580;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta80<F: Float>(t1088: F, t1653: F, t123: F, t1087: F, t423: F, t1086: F, t1100: F, t1107: F, t1113: F, t136: F, t1105: F, t1112: F, t1118: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1654, t1655, t1657) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk578::<F>(t1088, t1653, t123, t1087);
        let (t1659, t1661) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk579::<F>(t1657, t423, t1086, t1655);
        let (t1662, t1665, t1667, t1668, t1670) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk580::<F>(t1100, t1661, t1107, t1113, t1653, t136, t1105, t1112, t1655);
        let t1671 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk581::<F>(t1118, t1670);
    (t1654, t1655, t1657, t1659, t1661, t1662, t1665, t1667, t1668, t1670, t1671)
}
