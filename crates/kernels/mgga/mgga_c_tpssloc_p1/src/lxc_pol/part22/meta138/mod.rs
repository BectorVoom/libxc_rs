//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk895;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk896;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta138<F: Float>(t4195: F, t607: F, t4194: F, t1474: F, t172: F, t763: F, t185: F, t3966: F, t707: F, t1471: F, t706: F, t708: F, t1462: F, t2427: F, t2373: F, t2377: F, t2408: F, t4097: F, t4099: F, t4100: F, t4103: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4196, t4198, t4199) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk895::<F>(t4195, t607, t4194, t1474, t172);
        let (t4200, t4201, t4202, t4204, t4205) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk896::<F>(t4199, t763, t185, t3966, t707, t1471, t706);
        let (t4207, t4209, t4210) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk897::<F>(t4205, t708, t1462, t2427, t2373, t2377, t2408, t4097, t4099, t4100, t4103, t4198, t4201, t4204);
    (t4196, t4198, t4199, t4200, t4201, t4202, t4204, t4205, t4207, t4209, t4210)
}
