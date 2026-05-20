//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1295;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1296;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta222<F: Float>(t2239: F, t601: F, t83: F, t84: F, t85: F, t24: F, t2241: F, t645: F, t2307: F, t607: F, t65: F, t67: F, t1864: F, t2250: F, t2244: F, t628: F, t584: F, t9212: F, t25: F, t28: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9231, t9238, t9239, t9240, t9243, t9247) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1295::<F>(t2239, t601, t83, t84, t85, t24, t2241, t645, t2307, t607, t65, t67);
        let (t9248, t9251, t9256, t9257) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1296::<F>(t1864, t2250, t2244, t628, t584, t9212);
        let t9258 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1297::<F>(t25, t28, t9257, zeta_threshold);
    (t9231, t9238, t9239, t9240, t9243, t9247, t9248, t9251, t9256, t9257, t9258)
}
