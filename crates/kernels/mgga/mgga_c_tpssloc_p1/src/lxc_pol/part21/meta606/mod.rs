//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2365;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2366;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2367;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta606<F: Float>(t2511: F, t39377: F, t39378: F, t1294: F, t12088: F, t2535: F, t2504: F, t2368: F, t746: F, t268: F, t676: F, t9478: F, t9482: F, t9474: F, t9821: F, t2409: F, t2413: F, t125: F, t39253: F, t2414: F, t9479: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39381, t39382, t39384, t39387, t39389, t39391, t39393, t39397) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2365::<F>(t2511, t39377, t39378, t1294, t12088, t2535, t2504, t2368, t746, t268, t676, t9478, t9482);
        let t39400 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2366::<F>(t268, t9474, t9821);
        let t39408 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2367::<F>(t2409, t2413, t125, t39253);
        let t39411 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2368::<F>(t2414, t39253, t9479);
    (t39381, t39382, t39384, t39387, t39389, t39391, t39393, t39397, t39400, t39408, t39411)
}
