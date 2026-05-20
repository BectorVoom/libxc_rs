//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta121 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk813;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk814;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk815;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk816;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk817;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk818;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk819;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta121<F: Float>(t3242: F, t3439: F, t121: F, t486: F, t1216: F, t248: F, t1213: F, t478: F, t483: F, t3068: F, t1244: F, t1230: F, t820: F, t1089: F, t415: F, t61: F, t1236: F, t225: F, t1239: F, t496: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3560, t3570) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk813::<F>(t3242, t3439, t121, t486);
        let t3572 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk814::<F>(t1216, t248, t3570);
        let (t3573, t3575, t3576) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk815::<F>(t1213, t3572, t478, t483, t3068);
        let t3577 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk816::<F>(t1244, t3576);
        let t3578 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk817::<F>(t1230, t820);
        let t3584 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk818::<F>(t1089, t415);
        let (t3585, t3593) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk819::<F>(t3584, t61, t1236, t225);
        let t3598 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk820::<F>(t1239, t496, t68);
    (t3560, t3570, t3572, t3573, t3575, t3576, t3577, t3578, t3584, t3585, t3593, t3598)
}
