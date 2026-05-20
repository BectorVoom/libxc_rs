//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta87 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk562;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk563;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk564;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk565;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk566;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk567;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta87<F: Float>(t28: F, t265: F, t504: F, t1256: F, t1534: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F, t1763: F, t193: F, t336: F, t1409: F, t1649: F, t506: F, t52: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1647: F, t25: F, t1268: F, t1442: F, t1458: F, t1408: F, t514: F, t517: F, t157: F, t184: F, t17: F, t182: F, t1298: F, t1302: F, t210: F, t214: F, t1313: F, t1315: F, t1322: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1768, t1773) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk562::<F>(t28, t265, t504, t1256, t1534, t1659, t1673, t1699, t1701, t1705, t1763, t193, t336, t1409, t1649, t506, t52, dens_threshold, rho1, zeta_threshold);
        let t1774 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk563::<F>(t1647, t1773);
        let (t1778, t1787) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk564::<F>(t25, t28, t1268, t1442, t1458, t1408, t514, t1649, t517, t157, zeta_threshold);
        let t1788 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk565::<F>(t1787, t184);
        let (t1789, t1791, t1799) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk566::<F>(t25, t28, t17, t1788, t1787, t182, t1298, t1408, t1302, t1649, zeta_threshold);
        let (t1804, t1807) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk567::<F>(t1799, t210, t214, t1313, t1315, t1322);
    (t1768, t1774, t1778, t1787, t1788, t1789, t1791, t1799, t1804, t1807)
}
