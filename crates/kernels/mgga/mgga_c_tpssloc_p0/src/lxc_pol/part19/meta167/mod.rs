//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk795;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk796;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta167<F: Float>(t2566: F, t786: F, t2578: F, t2570: F, t792: F, t118: F, t2379: F, t794: F, t2553: F, t2576: F, t154: F, t845: F, t205: F, t210: F, t214: F, t9458: F, t213: F, t776: F, t221: F, t59: F, t8705: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9546, t9547, t9551, t9552, t9555, t9556, t9558) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk795::<F>(t2566, t786, t2578, t2570, t792, t118, t2379, t794, t2553, t2576, t154, t845);
        let (t9559, t9561, t9566, t9569) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk796::<F>(t205, t9558, t210, t214, t9458, t213, t776, t221, t2553, t59, t8705);
    (t9546, t9547, t9551, t9552, t9555, t9556, t9558, t9559, t9561, t9566, t9569)
}
