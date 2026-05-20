//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta304<F: Float>(t11135: F, t11203: F, t1128: F, t3324: F, t1124: F, t3356: F, t3355: F, t432: F, t427: F) -> (F, F, F, F, F, F) {
        let (t11369, t11372, t11410, t11415, t11419, t11420) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1649::<F>(t11135, t11203, t1128, t3324, t1124, t3356, t3355, t432, t427);
    (t11369, t11372, t11410, t11415, t11419, t11420)
}
