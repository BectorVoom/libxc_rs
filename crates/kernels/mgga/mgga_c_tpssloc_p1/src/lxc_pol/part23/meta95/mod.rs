//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk535;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk536;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta95<F: Float>(t3311: F, t409: F, t422: F, t3236: F, t1127: F, t432: F, t427: F, t3293: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3312, t3313) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk535::<F>(t3311, t409);
        let (t3314, t3315) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk536::<F>(t422);
        let (t3319, t3330, t3331, t3332, t3339, t3346, t3355) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk537::<F>(t3236, t1127, t432, t427, t3293);
    (t3312, t3313, t3314, t3315, t3319, t3330, t3331, t3332, t3339, t3346, t3355)
}
