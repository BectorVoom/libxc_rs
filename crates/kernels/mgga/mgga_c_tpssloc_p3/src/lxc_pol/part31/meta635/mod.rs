//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1899;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta635<F: Float>(t22633: F, t26338: F, t90566: F, t22751: F, t28213: F, t28210: F, t28233: F, t6883: F, t22674: F, t28232: F, t6897: F, t28195: F, t22635: F, t26337: F, t5353: F, t5325: F, t90488: F, t1307: F, t567: F, t6330: F, t90591: F, t28199: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97527, t97529, t97537, t97548, t97571, t97573) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1899::<F>(t22633, t26338, t90566, t22751, t28213, t28210, t28233, t6883, t22674, t28232, t6897, t28195);
        let (t97577, t97583, t97588, t97599) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1900::<F>(t22633, t22635, t26337, t5353, t5325, t90488, t1307, t567, t6330, t90591, t28199, t6897, t794);
    (t97527, t97529, t97537, t97548, t97571, t97573, t97577, t97583, t97588, t97599)
}
