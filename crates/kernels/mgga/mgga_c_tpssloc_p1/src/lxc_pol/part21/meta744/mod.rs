//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2612;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta744<F: Float>(t11797: F, t5005: F, t1174: F, t5045: F, t698: F, t3540: F, t4966: F, t11647: F, t1744: F, t11697: F, t15469: F, t3577: F, t11801: F, t15032: F, t3576: F, t11713: F, t11716: F, t53081: F, t11786: F, t5024: F, t3032: F, t52434: F, t3505: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53267, t53270, t53272, t53274, t53287) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2612::<F>(t11797, t5005, t1174, t5045, t698, t3540, t4966, t11647, t1744, t11697, t15469, t3577);
        let (t53291, t53322, t53336, t53360, t53371, t53372) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2613::<F>(t11801, t5005, t15032, t3576, t11713, t11716, t53081, t11786, t5024, t3032, t52434, t3505);
    (t53267, t53270, t53272, t53274, t53287, t53291, t53322, t53336, t53360, t53371, t53372)
}
