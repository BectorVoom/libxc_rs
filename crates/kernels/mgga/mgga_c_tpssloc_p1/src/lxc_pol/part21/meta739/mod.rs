//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta739 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2602;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta739<F: Float>(t10471: F, t52834: F, t11737: F, t11651: F, t15507: F, t13969: F, t15621: F, t3506: F, t11791: F, t5005: F, t11697: F, t15477: F, t3577: F, t11677: F, t15027: F, t3575: F, t373: F, t470: F, t493: F, t1214: F, t820: F, t3624: F, t52627: F, t11745: F, t15503: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52835, t52836, t52845, t52859, t52872, t52875) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2602::<F>(t10471, t52834, t11737, t11651, t15507, t13969, t15621, t3506, t11791, t5005, t11697, t15477, t3577);
        let (t52879, t52893, t52897, t52903, t52906) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2603::<F>(t11677, t15027, t3575, t373, t470, t493, t1214, t820, t3624, t52627, t11745, t15503);
    (t52835, t52836, t52845, t52859, t52872, t52875, t52879, t52893, t52897, t52903, t52906)
}
