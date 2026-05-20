//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta4 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk31;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk32;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk33;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk34;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk35;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk36;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk37;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk38;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta4<F: Float>(t40: F, t52: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t67 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk31::<F>();
        let t68 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk32::<F>();
        let (t71, t72) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk33::<F>(t68, t67);
        let t73 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk34::<F>(t40);
        let (t74, t75) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk35::<F>(t40, t73);
        let t76 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk36::<F>(t52);
        let (t77, t78) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk37::<F>(t52, t76);
        let (t79, t80) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk38::<F>(t75, t78, t72);
    (t67, t68, t71, t72, t73, t74, t75, t76, t77, t78, t79, t80)
}
