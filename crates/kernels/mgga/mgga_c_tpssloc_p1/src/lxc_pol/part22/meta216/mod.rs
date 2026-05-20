//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta216 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1244;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1245;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1246;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1247;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta216<F: Float>(t562: F, t6361: F, t1807: F, t1834: F, t119: F, t6330: F, t210: F, t6347: F, t225: F, t554: F, t1824: F, t3792: F, t1343: F, t820: F, t1799: F, t550: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6362, t6364, t6370, t6371, t6374, t6375, t6378) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1244::<F>(t562, t6361, t1807, t1834, t119, t6330, t210, t6347, t225);
        let (t6379, t6387) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1245::<F>(t554, t6378, t1824);
        let t6388 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1246::<F>(t3792, t6387);
        let t6390 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1247::<F>(t1343, t6388, t820);
        let t6394 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1248::<F>(t1799, t550);
    (t6362, t6364, t6370, t6371, t6374, t6375, t6378, t6379, t6387, t6388, t6390, t6394)
}
