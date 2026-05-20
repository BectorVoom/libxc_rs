//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta71 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk428;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk429;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk430;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk431;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk432;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta71<F: Float>(t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1297: F, t1390: F, t1789: F, t1791: F, t1799: F, t1845: F, t193: F, t533: F, t680: F, t705: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t510: F, t513: F, t574: F, t652: F, t3: F, t1401: F, t1458: F, t577: F, t71: F, t79: F, t202: F, t154: F, t204: F, t119: F, t210: F, t201: F, t243: F, t335: F, t371: F, t532: F, t556: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1849 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk428::<F>(t1274, t1276, t1288, t1293, t1296, t1297, t1390, t1789, t1791, t1799, t1845, t193, t533, t680, t705);
        let t1851 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk429::<F>(t113, t1442, t1459, t1774, t1778, t1849, t510, t513, t574, t652);
        let (t1852, t1858, t1864, t1877, t1878, t1887) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk430::<F>(t1851, t3, t1401, t1458, t577, t71, t79, t193, t202, t154, t204, t119, t210);
        let t1891 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk431::<F>(t201, t243);
        let t1932 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk432::<F>(t335, t371);
        let t1995 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk433::<F>(t532, t556);
    (t1849, t1851, t1852, t1858, t1864, t1877, t1878, t1887, t1891, t1932, t1995)
}
