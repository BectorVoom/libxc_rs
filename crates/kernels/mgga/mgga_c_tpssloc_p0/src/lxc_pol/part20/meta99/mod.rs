//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk670;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk671;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta99<F: Float>(t2471: F, t731: F, t723: F, t159: F, t167: F, t2461: F, t676: F, t682: F, t268: F, t703: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2472, t2475) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk670::<F>(t2471, t731, t723);
        let (t2476, t2477, t2478, t2479) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk671::<F>(t2475, t159, t167);
        let (t2480, t2483, t2486) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk672::<F>(t2461, t2479, t676, t682, t268, t703);
    (t2472, t2475, t2476, t2477, t2478, t2479, t2480, t2483, t2486)
}
