//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk614;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta111<F: Float>(t3040: F, t3131: F, t1021: F, t248: F) -> (F, F) {
        let t3132 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk614::<F>(t3040, t3131);
        let t3134 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk615::<F>(t1021, t248, t3132);
    (t3132, t3134)
}
