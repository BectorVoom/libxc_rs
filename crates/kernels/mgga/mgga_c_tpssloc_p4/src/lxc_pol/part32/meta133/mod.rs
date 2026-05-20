//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk746;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk747;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk748;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk749;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk750;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk751;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk752;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta133<F: Float>(t1208: F, t476: F, t478: F, t3036: F, t483: F, t3500: F, t475: F, t1210: F, t121: F, t1229: F, t1090: F, t248: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t3502 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk746::<F>(t1208, t476);
        let t3503 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk747::<F>(t3502, t478);
        let (t3504, t3505, t3506) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk748::<F>(t3036, t483, t3503, t3500);
        let t3508 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk749::<F>(t475);
        let (t3514, t3515) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk750::<F>(t1210, t3504, t3500);
        let t3521 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk751::<F>(t121, t1229);
        let t3523 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk752::<F>(t1090, t248, t3521);
    (t3502, t3503, t3504, t3505, t3506, t3508, t3514, t3515, t3521, t3523)
}
