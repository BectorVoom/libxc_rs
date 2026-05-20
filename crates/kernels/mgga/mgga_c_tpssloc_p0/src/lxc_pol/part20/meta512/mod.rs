//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2029;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2030;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta512<F: Float>(t3691: F, t9494: F, t2508: F, t2369: F, t2511: F, t1294: F, t12088: F, t2535: F, t2504: F, t2368: F, t746: F, t268: F, t676: F, t9478: F, t9482: F, t9474: F, t9821: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39374, t39377, t39378) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2029::<F>(t3691, t9494, t2508, t2369);
        let (t39381, t39382, t39384, t39387, t39389, t39391, t39393, t39397) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2030::<F>(t2511, t39377, t39378, t1294, t12088, t2535, t2504, t2368, t746, t268, t676, t9478, t9482);
        let t39400 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2031::<F>(t268, t9474, t9821);
    (t39374, t39377, t39378, t39381, t39382, t39384, t39387, t39389, t39391, t39393, t39397, t39400)
}
