//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta312 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1670;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1671;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1672;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1673;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta312<F: Float>(t1208: F, t478: F, t10477: F, t483: F, t11713: F, t3508: F, t475: F, t3503: F, t11708: F, t3514: F, t1210: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11714, t11715, t11716, t11717, t11718, t11719) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1670::<F>(t1208, t478, t10477, t483, t11713);
        let t11721 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1671::<F>(t3508, t475);
        let (t11727, t11728) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1672::<F>(t11717, t3503, t11713);
        let t11734 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1673::<F>(t11708, t3514);
        let (t11737, t11738) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1674::<F>(t11717, t1210, t11713);
    (t11714, t11715, t11716, t11718, t11719, t11721, t11727, t11728, t11734, t11737, t11738)
}
