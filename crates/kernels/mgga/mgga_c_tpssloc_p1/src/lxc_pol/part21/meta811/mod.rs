//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta811 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2843;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2844;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2845;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2846;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2847;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2848;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta811<F: Float>(t17156: F, t2250: F, t123: F, t2768: F, t12606: F, t4337: F, t41741: F, t59688: F, t59692: F, t59694: F, t59698: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t10216: F, t2244: F, t5398: F, t10564: F, t16558: F, t2775: F, t607: F, t882: F, t47774: F, t47779: F, t55716: F, t47775: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t59715, t59717) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2843::<F>(t17156, t2250, t123, t2768);
        let (t59719, t59721) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2844::<F>(t12606, t4337, t123, t2768);
        let t59723 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2845::<F>(t41741, t59688, t59692, t59694, t59698, t59700, t59702, t59704, t59708, t59713, t59717, t59721);
        let (t59725, t59727) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2846::<F>(t10216, t2244, t5398, t10564, t123);
        let (t59730, t59732) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2847::<F>(t16558, t2775, t607, t123, t882);
        let t59735 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2848::<F>(t47774, t47779, t55716);
        let t59738 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2849::<F>(t47774, t47775, t55716);
    (t59715, t59717, t59719, t59721, t59723, t59725, t59727, t59730, t59732, t59735, t59738)
}
