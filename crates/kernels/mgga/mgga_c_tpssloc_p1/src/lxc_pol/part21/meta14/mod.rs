//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta14 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk107;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk108;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk109;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk110;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk111;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk112;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk113;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk114;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk115;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk116;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta14<F: Float>(t15: F, t61: F, t154: F, t201: F, t132: F, t67: F, t120: F, t219: F, t222: F, t238: F, t218: F, t225: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t240 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk107::<F>(t15, t61);
        let t241 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk108::<F>(t154);
        let t242 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk109::<F>(t240, t241);
        let t243 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk110::<F>(t201);
        let t244 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk111::<F>(t243);
        let t246 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk112::<F>(t132);
        let t247 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk113::<F>(t246, t67);
        let t248 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk114::<F>(t120, t247);
        let t249 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk115::<F>(t242, t244, t248);
        let t252 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk116::<F>(t219, t222, t238, t249);
        let (t253, t254) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk117::<F>(t218, t252, t225, t68);
    (t240, t241, t242, t243, t244, t246, t247, t248, t249, t252, t253, t254)
}
