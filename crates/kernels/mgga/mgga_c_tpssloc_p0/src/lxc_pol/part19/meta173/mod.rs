//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta173<F: Float>(t2617: F, t2638: F, t831: F, t2639: F, t2681: F, t184: F, t2250: F, t607: F, t4194: F, t116: F, t126: F, t136: F) -> (F, F, F, F, F, F, F) {
        let (t9674, t9675, t9679, t9681, t9682, t9684, t9688) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk807::<F>(t2617, t2638, t831, t2639, t2681, t184, t2250, t607, t4194, t116, t126, t136);
    (t9674, t9675, t9679, t9681, t9682, t9684, t9688)
}
