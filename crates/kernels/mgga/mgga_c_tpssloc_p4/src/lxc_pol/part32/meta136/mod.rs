//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta136 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk759;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk760;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk761;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk762;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk763;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk764;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk765;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta136<F: Float>(t1089: F, t415: F, t61: F, t1236: F, t225: F, t1239: F, t496: F, t68: F, t1243: F, t3534: F, t3032: F, t3502: F, t3499: F, t1932: F, t3508: F, t1209: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3584 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk759::<F>(t1089, t415);
        let (t3585, t3593) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk760::<F>(t3584, t61, t1236, t225);
        let (t3597, t3598) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk761::<F>(t1239, t496, t68);
        let t3604 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk762::<F>(t1243, t3534);
        let (t3609, t3610) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk763::<F>(t3032, t3502, t3499);
        let t3612 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk764::<F>(t1932, t3508);
        let (t3623, t3624) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk765::<F>(t1209, t3032, t3499);
        let t3625 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk766::<F>(t1932, t475);
    (t3584, t3585, t3593, t3597, t3598, t3604, t3609, t3610, t3612, t3623, t3624, t3625)
}
