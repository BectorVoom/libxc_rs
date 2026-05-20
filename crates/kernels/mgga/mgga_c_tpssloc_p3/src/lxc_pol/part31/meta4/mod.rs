//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta4 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk22;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk23;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk24;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk25;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk26;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk27;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk28;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk29;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk30;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk31;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk32;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta4<F: Float>(t3: F, t10: F, t59: F, t39: F, t44: F, t51: F, t56: F, t33: F, t40: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t60 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk22::<F>(t3);
        let t61 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk23::<F>(t60);
        let t63 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk24::<F>(t10, t61);
        let t64 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk25::<F>(t59, t63);
        let t65 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk26::<F>(t39, t44, t51, t56, t64);
        let t66 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk27::<F>(t33, t65);
        let t67 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk28::<F>();
        let t68 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk29::<F>();
        let t71 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk30::<F>(t68);
        let t72 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk31::<F>(t67, t71);
        let t73 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk32::<F>(t40);
    (t60, t61, t63, t64, t65, t66, t67, t68, t71, t72, t73)
}
