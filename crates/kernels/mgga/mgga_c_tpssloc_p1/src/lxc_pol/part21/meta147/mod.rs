//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta147 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk957;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk958;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk959;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk960;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk961;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk962;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk963;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk964;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk965;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk966;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta147<F: Float>(t1208: F, t476: F, t478: F, t3036: F, t483: F, t3500: F, t1215: F, t475: F, t1214: F, t248: F, t1210: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3502, t3503) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk957::<F>(t1208, t476, t478);
        let (t3504, t3505) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk958::<F>(t3036, t483, t3503);
        let t3506 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk959::<F>(t3500, t3505);
        let t3507 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk960::<F>(t1215);
        let t3508 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk961::<F>(t475);
        let t3509 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk962::<F>(t3507, t3508);
        let t3511 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk963::<F>(t1214, t248, t3509);
        let t3514 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk964::<F>(t1210, t3504);
        let t3515 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk965::<F>(t3500, t3514);
        let t3516 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk966::<F>(t3507, t475);
        let t3518 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk967::<F>(t1214, t248, t3516);
    (t3502, t3503, t3505, t3506, t3507, t3508, t3509, t3511, t3514, t3515, t3516, t3518)
}
