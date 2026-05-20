//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta74 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk518;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk519;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk520;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk521;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk522;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta74<F: Float>(t1527: F, t858: F, t1493: F, t1520: F, t259: F, t855: F, t1464: F, t1473: F, t1476: F, t1484: F, t193: F, t202: F, t680: F, t705: F, t752: F, t760: F, t765: F, t766: F, t870: F, t1409: F, t883: F, t882: F, t123: F, t881: F, t291: F, t880: F) -> (F, F, F, F, F, F, F, F, F) {
        let t1528 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk518::<F>(t1527, t858);
        let t1530 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk519::<F>(t1493, t1520, t1528, t259, t855);
        let t1534 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk520::<F>(t1464, t1473, t1476, t1484, t1530, t193, t202, t680, t705, t752, t760, t765, t766, t870);
        let t1539 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk521::<F>(t1409, t883);
        let (t1540, t1541, t1543) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk522::<F>(t1539, t882, t123, t881);
        let (t1545, t1547) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk523::<F>(t1543, t291, t1541, t880);
    (t1528, t1530, t1534, t1539, t1540, t1541, t1543, t1545, t1547)
}
