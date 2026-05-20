//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta318<F: Float>(t11697: F, t4949: F, t3577: F, t3431: F, t4729: F, t1174: F, t1011: F, t15031: F, t1212: F, t1226: F, t4965: F) -> (F, F, F, F, F, F, F) {
        let (t15572, t15574, t15578, t15580, t15590, t15591, t15594) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1500::<F>(t11697, t4949, t3577, t3431, t4729, t1174, t1011, t15031, t1212, t1226, t4965);
    (t15572, t15574, t15578, t15580, t15590, t15591, t15594)
}
