//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta31 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk228;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk229;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk230;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk231;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk232;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta31<F: Float>(t31: F, t607: F, t65: F, t34: F, t36: F, rho0: F, sigma0: F, t43: F, t55: F, t583: F, t61: F, t59: F, t39: F, t44: F, t51: F, t33: F, t40: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t608, t609, t612, t614, t615) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk228::<F>(t31, t607, t65, t34, t36, rho0, sigma0);
        let (t618, t621, t625) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk229::<F>(t43, t607, t55, t583, t61);
        let t626 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk230::<F>(t59, t625);
        let t628 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk231::<F>(t626, t39, t44, t51, t615, t618, t621);
        let (t629, t632) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk232::<F>(t33, t628, t40);
        let t634 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk233::<F>(t632, t73);
    (t608, t609, t612, t614, t615, t618, t625, t626, t628, t629, t632, t634)
}
