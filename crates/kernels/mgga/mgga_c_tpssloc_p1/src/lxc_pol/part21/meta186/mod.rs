//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta186 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1171;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1172;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1173;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta186<F: Float>(t4496: F, t4497: F, t959: F, t1592: F, t2970: F, t973: F, t2978: F, t60: F, t344: F, t4338: F, t1409: F, t2989: F, t2988: F, t2987: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4498, t4500, t4506, t4507, t4509) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1171::<F>(t4496, t4497, t959, t1592, t2970, t973, t2978, t60);
        let t4510 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1172::<F>(t344, t4509);
        let (t4511, t4514) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1173::<F>(t4338, t4510, t1409, t2989);
        let (t4515, t4518) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1174::<F>(t2988, t4514, t2987, t344);
    (t4498, t4500, t4506, t4507, t4509, t4510, t4511, t4514, t4515, t4518)
}
