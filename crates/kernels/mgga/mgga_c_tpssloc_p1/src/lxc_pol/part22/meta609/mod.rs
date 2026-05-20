//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2136;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta609<F: Float>(t1667: F, t9709: F, t2403: F, t4778: F, t2394: F, t4725: F, t4730: F) -> (F, F, F, F, F) {
        let (t50846, t50853, t50854, t50919) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2136::<F>(t1667, t9709, t2403, t4778, t2394, t4725);
        let t50948 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2137::<F>(t2394, t4730);
    (t50846, t50853, t50854, t50919, t50948)
}
