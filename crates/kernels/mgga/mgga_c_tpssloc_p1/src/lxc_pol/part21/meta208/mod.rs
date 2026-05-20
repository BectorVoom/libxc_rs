//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta208 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1278;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1279;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1280;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1281;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta208<F: Float>(t1243: F, t5000: F, t1215: F, t3612: F, t1755: F, t1235: F, t1734: F, t1246: F, t491: F, t5011: F, t1932: F, t475: F, t1751: F, t493: F, t5052: F, t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1756: F, t1758: F, t3604: F, t3610: F, t3624: F, t470: F, t494: F, t4964: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5064 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1278::<F>(t1243, t5000);
        let t5068 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1279::<F>(t1215, t3612);
        let (t5069, t5072) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1280::<F>(t1755, t5068, t1235, t1734);
        let (t5073, t5075, t5076, t5079) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1281::<F>(t1246, t5072, t491, t5011, t1215, t1932, t475);
        let (t5080, t5084, t5086, t5088) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1282::<F>(t1755, t5079, t1215, t1751, t1246, t493, t5052, t1201, t1244, t1247, t1249, t1729, t1756, t1758, t3604, t3610, t3624, t470, t494, t4964, t5064, t5069, t5073, t5076);
    (t5064, t5068, t5069, t5072, t5073, t5075, t5076, t5079, t5080, t5084, t5086, t5088)
}
