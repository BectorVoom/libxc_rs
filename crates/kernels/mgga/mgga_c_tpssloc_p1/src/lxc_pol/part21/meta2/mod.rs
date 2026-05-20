//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta2 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk17;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk18;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk19;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta2<F: Float>(rho0: F, sigma0: F, t31: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t34, t35, t36, t38, t39) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk17::<F>(rho0, sigma0);
        let t40 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk18::<F>(t31);
        let (t41, t42, t43) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk19::<F>(t40);
    (t34, t35, t36, t38, t39, t40, t41, t42, t43)
}
