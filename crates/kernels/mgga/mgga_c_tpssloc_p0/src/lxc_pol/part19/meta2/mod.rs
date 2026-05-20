//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta2 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk18;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk19;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk20;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk21;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk22;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk23;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta2<F: Float>(t34: F, t36: F, sigma0: F, t31: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t38, t39) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk18::<F>(t34, t36, sigma0);
        let t40 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk19::<F>(t31);
        let (t41, t42, t43) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk20::<F>(t40);
        let t44 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk21::<F>(t41, t43);
        let (t46, t47, t48, t51) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk22::<F>(rho1, sigma2);
        let t52 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk23::<F>(t31);
    (t38, t39, t40, t41, t42, t43, t44, t46, t47, t48, t51, t52)
}
