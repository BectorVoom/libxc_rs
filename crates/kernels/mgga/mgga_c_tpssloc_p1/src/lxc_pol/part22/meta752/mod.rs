//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2525;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2526;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2527;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2528;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta752<F: Float>(t47774: F, t50998: F, t68513: F, t43816: F, t44348: F, t51565: F, t51574: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t71166: F, t71170: F, t71174: F, t71179: F, t71183: F, t71187: F, t71191: F, t71195: F, t71199: F, t71203: F, t423: F, t71162: F, t1157: F, t1164: F, t21938: F, t3375: F, t1254: F, t19270: F, t4700: F, t5091: F, t71095: F, t71097: F, t71101: F, t71106: F, t71109: F, t71112: F, t71114: F, t71118: F, t4861: F, t64525: F, t21833: F, t3411: F, t18786: F, t4874: F, t21826: F, t300: F, t1166: F, t22236: F, t4883: F) -> (F, F, F, F, F, F, F, F, F) {
        let t71206 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2525::<F>(t47774, t50998, t68513);
        let t71214 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2526::<F>(t43816, t44348, t51565, t51574, t63361, t63382, t63384, t63398, t63400, t71166, t71170, t71174, t71179, t71183, t71187, t71191, t71195, t71199, t71203, t71206);
        let (t71217, t71221, t71222) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2527::<F>(t423, t71162, t71214, t1157, t1164, t21938, t3375, t1254, t19270, t4700, t5091, t71095, t71097, t71101, t71106, t71109, t71112, t71114, t71118);
        let (t71225, t71227, t71230, t71233, t71236) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2528::<F>(t1164, t4861, t64525, t21833, t3411, t18786, t4874, t21826, t300, t1166, t22236, t4883);
    (t71206, t71217, t71221, t71222, t71225, t71227, t71230, t71233, t71236)
}
