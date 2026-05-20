//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk576;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk577;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk578;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk579;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk580;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk581;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk582;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta90<F: Float>(t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1297: F, t1390: F, t1789: F, t1791: F, t1799: F, t1845: F, t193: F, t533: F, t680: F, t705: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t510: F, t513: F, t574: F, t652: F, t3: F, t1401: F, t1458: F, t577: F, t33: F, t605: F, t71: F, t79: F, t109: F, t107: F, t63: F, t202: F, t154: F, t204: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1849 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk576::<F>(t1274, t1276, t1288, t1293, t1296, t1297, t1390, t1789, t1791, t1799, t1845, t193, t533, t680, t705);
        let t1851 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk577::<F>(t113, t1442, t1459, t1774, t1778, t1849, t510, t513, t574, t652);
        let (t1852, t1858, t1860) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk578::<F>(t1851, t3, t1401, t1458, t577, t33, t605);
        let t1864 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk579::<F>(t71, t79);
        let t1873 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk580::<F>(t109, t107, t63);
        let t1874 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk581::<F>(t1873, t510);
        let (t1876, t1877) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk582::<F>(t1874, t652, t193, t202);
        let t1878 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk583::<F>(t154, t204);
    (t1849, t1851, t1852, t1858, t1860, t1864, t1873, t1874, t1876, t1877, t1878)
}
