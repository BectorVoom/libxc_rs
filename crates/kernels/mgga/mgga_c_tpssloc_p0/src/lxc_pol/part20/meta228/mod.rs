//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta228 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1309;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1310;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1311;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1312;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta228<F: Float>(t1268: F, t2314: F, t2363: F, t5113: F, t671: F, t9347: F, t9348: F, t9351: F, t9416: F, t195: F, t40: F, t2433: F, t607: F, t52: F, t2250: F, t73: F, t9258: F, t9288: F, t197: F, t2440: F, t76: F, t145: F, zeta_threshold: F, t185: F, t138: F, t2409: F, t125: F, t2412: F, t701: F, t2414: F, t2379: F, t776: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9419, t9427, t9430) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1309::<F>(t1268, t2314, t2363, t5113, t671, t9347, t9348, t9351, t9416, t195, t40, t2433, t607);
        let (t9438, t9448, t9449) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1310::<F>(t40, t52, t2250, t73, t9258, t9288, t9427, t9430, t197, t2440, t607, t76, t145, zeta_threshold);
        let (t9450, t9452, t9453, t9454, t9455, t9457) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1311::<F>(t185, t9449, t138, t2409, t125, t2412, t701, t2414);
        let t9458 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1312::<F>(t2379, t776);
    (t9419, t9427, t9438, t9448, t9449, t9450, t9452, t9453, t9454, t9455, t9457, t9458)
}
