//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1569;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1570;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta312<F: Float>(t3355: F, t432: F, t427: F, t11306: F, t3359: F, t1094: F, t3263: F, t3266: F, t1118: F, t11191: F, t3313: F, t1157: F, t3395: F, t3403: F, t1155: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11419, t11420) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1569::<F>(t3355, t432, t427);
        let (t11421, t11424, t11426, t11427, t11429, t11430, t11433, t11434) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1570::<F>(t11306, t3359, t1094, t3263, t3266, t1118, t11191, t3313, t1157, t3395, t3403, t1155);
    (t11419, t11420, t11421, t11424, t11426, t11427, t11429, t11430, t11433, t11434)
}
