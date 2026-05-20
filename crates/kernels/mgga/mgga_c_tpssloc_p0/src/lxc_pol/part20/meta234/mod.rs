//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1329;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1330;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta234<F: Float>(t2588: F, t9577: F, t21: F, t59: F, t207: F, t795: F, t4127: F, t787: F, t9526: F, t9529: F, t9540: F, t9542: F, t9544: F, t9547: F, t9552: F, t9556: F, t9559: F, t9561: F, t9566: F, t9572: F, t9574: F, t252: F, t2591: F, t852: F, t225: F, t2711: F, t2594: F, t2690: F, t841: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9579, t9580, t9583, t9584) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1329::<F>(t2588, t9577, t21, t59, t207, t795, t4127, t787, t9526, t9529, t9540, t9542, t9544, t9547, t9552, t9556, t9559, t9561, t9566, t9572, t9574);
        let (t9585, t9587, t9590, t9593, t9600, t9601) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1330::<F>(t252, t9584, t2591, t852, t225, t2711, t2594, t2690, t841, t812);
    (t9579, t9580, t9583, t9584, t9585, t9587, t9590, t9593, t9600, t9601)
}
