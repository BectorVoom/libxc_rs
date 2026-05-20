//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk565;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk566;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk567;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk568;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta79<F: Float>(t265: F, t394: F, t1052: F, t1604: F, t1626: F, t1635: F, t388: F, t1070: F, t1534: F, t1545: F, t1559: F, t1585: F, t1587: F, t1591: F, t193: F, t336: F, t25: F, t1408: F, t1409: F, t396: F, t40: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1089: F, t1088: F, t123: F, t1087: F, t423: F, t1086: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1637, t1642) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk565::<F>(t265, t394, t1052, t1604, t1626, t1635, t388, t1070, t1534, t1545, t1559, t1585, t1587, t1591, t193, t336);
        let (t1647, t1649) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk566::<F>(t25, t1408, t1409, t1534, t1642, t265, t396, t40, dens_threshold, rho0, zeta_threshold);
        let t1653 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk567::<F>(t1089, t1409);
        let (t1654, t1655, t1657) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk568::<F>(t1088, t1653, t123, t1087);
        let (t1659, t1661) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk569::<F>(t1657, t423, t1086, t1655);
    (t1637, t1642, t1647, t1649, t1653, t1654, t1655, t1657, t1659, t1661)
}
