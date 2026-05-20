//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta84 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk585;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk586;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk587;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk588;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk589;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk590;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk591;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta84<F: Float>(t1799: F, t210: F, t214: F, t1313: F, t1315: F, t1322: F, t562: F, t119: F, t225: F, t554: F, t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1789: F, t1791: F, t680: F, t705: F, t1347: F, t546: F, t548: F, t550: F, t1343: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1804, t1807) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk585::<F>(t1799, t210, t214, t1313, t1315, t1322);
        let (t1808, t1810) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk586::<F>(t1807, t562, t119, t1799);
        let (t1811, t1814) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk587::<F>(t1810, t210, t1807, t225);
        let (t1815, t1819) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk588::<F>(t1814, t554, t1274, t1276, t1288, t1293, t1296, t1789, t1791, t225, t680, t705);
        let t1821 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk589::<F>(t1347, t1799);
        let t1824 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk590::<F>(t1819, t1821, t546, t548);
        let t1825 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk591::<F>(t1824, t550);
        let t1827 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk592::<F>(t1343, t1825, t820);
    (t1804, t1807, t1808, t1810, t1811, t1814, t1815, t1819, t1821, t1824, t1825, t1827)
}
