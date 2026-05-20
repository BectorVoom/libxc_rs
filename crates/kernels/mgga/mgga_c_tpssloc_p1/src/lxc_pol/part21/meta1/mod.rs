//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta1 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk10;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk11;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk12;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk13;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk14;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk15;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk16;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta1<F: Float>(t10: F, t15: F, t11: F, t17: F, t19: F, t9: F, t5: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
        let (t20, t21) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk10::<F>(t10, t15);
        let t24 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk11::<F>(t11, t17, t19, t21, t9);
        let t25 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk12::<F>(t5);
        let (t27, t28) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk13::<F>(t25, t5, zeta_threshold);
        let t31 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk14::<F>(t25, t28, t27, t5, zeta_threshold);
        let t32 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk15::<F>(t31);
        let t33 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk16::<F>(t32);
    (t20, t21, t24, t25, t28, t31, t32, t33)
}
