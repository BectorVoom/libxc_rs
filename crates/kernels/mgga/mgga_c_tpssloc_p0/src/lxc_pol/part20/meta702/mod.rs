//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta702 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2671;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta702<F: Float>(t39611: F, t39620: F, t39628: F, t39630: F, t39632: F, t39634: F, t39636: F, t39642: F, t39644: F, t5154: F, t9722: F, t39659: F, t39615: F, t39639: F, t39655: F, t39658: F, t39844: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54442, t54443, t54444, t54445, t54446, t54447, t54448, t54449, t54450, t54452, t54453) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2671::<F>(t39611, t39620, t39628, t39630, t39632, t39634, t39636, t39642, t39644, t5154, t9722, t39659);
        let t54454 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2672::<F>(t39615, t39639, t39655, t39658, t39844, t54442, t54443, t54444, t54445, t54446, t54447, t54448, t54449, t54450, t54452, t54453);
    (t54442, t54443, t54444, t54445, t54446, t54447, t54448, t54449, t54450, t54452, t54453, t54454)
}
