//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta635<F: Float>(t2617: F, t9670: F, t831: F, t236: F, t40931: F, t2638: F, t9612: F, t10021: F, t812: F, t815: F, t2686: F, t9671: F) -> (F, F, F, F, F, F, F) {
        let (t41340, t41341, t41347, t41354, t41362, t41363, t41365) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2420::<F>(t2617, t9670, t831, t236, t40931, t2638, t9612, t10021, t812, t815, t2686, t9671);
    (t41340, t41341, t41347, t41354, t41362, t41363, t41365)
}
