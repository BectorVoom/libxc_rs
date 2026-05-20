//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta159 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1033;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1034;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1035;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1036;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1037;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1038;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1039;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1040;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta159<F: Float>(t240: F, t3788: F, t1336: F, t1351: F, t550: F, t1343: F, t820: F, t1339: F, t835: F, t1354: F, t242: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t3789 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1033::<F>(t240, t3788);
        let (t3790, t3791) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1034::<F>(t1336, t3789, t1351);
        let t3792 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1035::<F>(t550);
        let t3793 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1036::<F>(t3791, t3792);
        let t3795 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1037::<F>(t1343, t3793, t820);
        let t3798 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1038::<F>(t1339, t835);
        let t3799 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1039::<F>(t1336, t3798);
        let (t3800, t3802) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1040::<F>(t1354, t3799, t1339, t242);
        let t3803 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1041::<F>(t1336, t3802);
    (t3789, t3790, t3791, t3792, t3793, t3795, t3798, t3799, t3800, t3802, t3803)
}
