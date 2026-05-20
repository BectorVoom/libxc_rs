//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1905;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta602<F: Float>(t22633: F, t22637: F, t90566: F, t26331: F, t26333: F, t80650: F, t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F, t22635: F, t26332: F, t3734: F, t22916: F, t26193: F, t6888: F, t26354: F, t90506: F, t26211: F, t6883: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90568, t90571, t90582, t90584, t90591) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1905::<F>(t22633, t22637, t90566, t26331, t26333, t80650, t22724, t26474, t22751, t26194, t1887, t80830);
        let (t90594, t90598, t90602, t90604) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1906::<F>(t22635, t26332, t3734, t90591, t22916, t26193, t6888, t22633, t26354, t90506, t26211, t6883);
    (t90568, t90571, t90582, t90584, t90591, t90594, t90598, t90602, t90604)
}
