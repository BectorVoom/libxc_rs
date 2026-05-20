//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1494;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1495;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta488<F: Float>(t54389: F, t56185: F, t54392: F, t74072: F, t74074: F, t74077: F, t54411: F, t54412: F, t20416: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t5126: F, t5127: F, t6347: F, t54428: F, t193: F, t3918: F, t3924: F, t39490: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F, t39549: F, t39563: F, t5122: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t79904, t79905, t79906, t79907, t79908, t79909, t79910, t79914, t79915) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1494::<F>(t54389, t56185, t54392, t74072, t74074, t74077, t54411, t54412, t20416, t39411, t39463, t39468, t39472, t39476, t39483, t5126, t5127);
        let t79921 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1495::<F>(t6347);
        let (t79925, t79926) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1496::<F>(t54428, t193, t20416, t3918, t3924, t39490, t39496, t39499, t39502, t39505, t39508, t39518, t39521, t39529, t39539, t39549, t39563, t5122, t79921);
    (t79904, t79905, t79906, t79907, t79908, t79909, t79910, t79914, t79915, t79921, t79925, t79926)
}
