//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk533;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk534;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk535;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk536;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk537;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta87<F: Float>(t1246: F, t1755: F, t1751: F, t493: F, t1244: F, t1729: F, t470: F, t494: F, t1241: F, t265: F, t504: F, t1238: F, t1721: F, t1752: F, t498: F, t1256: F, t1534: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F, t193: F, t336: F, t28: F, t1409: F, t1649: F, t506: F, t52: F, t1647: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1268: F, t1442: F, t1458: F, t25: F, t1408: F, t514: F, t517: F, t157: F, t184: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1756, t1758, t1760, t1761) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk533::<F>(t1246, t1755, t1751, t493, t1244, t1729, t470, t494, t1241);
        let (t1763, t1768) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk534::<F>(t265, t504, t1238, t1721, t1752, t1761, t498, t1256, t1534, t1659, t1673, t1699, t1701, t1705, t193, t336);
        let t1774 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk535::<F>(t28, t1409, t1534, t1649, t1768, t265, t506, t52, t1647, dens_threshold, rho1, zeta_threshold);
        let t1778 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk536::<F>(t1268, t1442, t1458);
        let t1787 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk537::<F>(t25, t28, t1408, t514, t1649, t517, t157, zeta_threshold);
        let t1788 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk538::<F>(t1787, t184);
    (t1756, t1758, t1760, t1761, t1763, t1768, t1774, t1778, t1787, t1788)
}
