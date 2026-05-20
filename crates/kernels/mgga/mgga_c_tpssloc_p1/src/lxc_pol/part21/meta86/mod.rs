//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk616;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk617;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk618;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk619;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk620;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk621;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk622;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk623;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk624;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk625;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta86<F: Float>(t1819: F, t1821: F, t546: F, t548: F, t550: F, t1343: F, t820: F, t1367: F, t1799: F, t1315: F, t1327: F, t1341: F, t1360: F, t1363: F, t1811: F, t1815: F, t559: F, t539: F, t1380: F, t553: F, t1336: F, t1814: F, t544: F, t564: F, t1378: F, t1375: F, t1808: F, t568: F, t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1297: F, t1390: F, t1789: F, t1791: F, t193: F, t533: F, t680: F, t705: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1824 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk616::<F>(t1819, t1821, t546, t548);
        let t1825 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk617::<F>(t1824, t550);
        let t1827 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk618::<F>(t1343, t1825, t820);
        let t1831 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk619::<F>(t1367, t1799, t820);
        let t1834 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk620::<F>(t1315, t1327, t1341, t1360, t1363, t1811, t1815, t1827, t1831, t559);
        let (t1835, t1838) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk621::<F>(t1834, t539, t1380, t1825);
        let t1840 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk622::<F>(t1834, t553);
        let t1842 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk623::<F>(t1336, t1814, t1838, t1840, t544, t564);
        let t1843 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk624::<F>(t1378, t1842);
        let t1845 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk625::<F>(t1375, t1808, t1835, t1843, t568);
        let t1849 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk626::<F>(t1274, t1276, t1288, t1293, t1296, t1297, t1390, t1789, t1791, t1799, t1845, t193, t533, t680, t705);
    (t1824, t1825, t1827, t1831, t1834, t1835, t1838, t1840, t1842, t1843, t1845, t1849)
}
