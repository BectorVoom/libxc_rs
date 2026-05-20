//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta13 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk104;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk105;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk106;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk107;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk108;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk109;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk110;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk111;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk112;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk113;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta13<F: Float>(t59: F, t235: F, t226: F, t15: F, t61: F, t154: F, t201: F, t132: F, t67: F, t120: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t236 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk104::<F>(t59);
        let t237 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk105::<F>(t235, t236);
        let t238 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk106::<F>(t226, t237);
        let t240 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk107::<F>(t15, t61);
        let t241 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk108::<F>(t154);
        let t242 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk109::<F>(t240, t241);
        let t243 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk110::<F>(t201);
        let t244 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk111::<F>(t243);
        let t246 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk112::<F>(t132);
        let (t247, t248) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk113::<F>(t246, t67, t120);
        let t249 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk114::<F>(t242, t244, t248);
    (t236, t237, t238, t240, t241, t242, t243, t244, t246, t247, t248, t249)
}
