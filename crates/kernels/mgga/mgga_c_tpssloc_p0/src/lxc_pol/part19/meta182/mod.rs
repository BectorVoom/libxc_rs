//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk832;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta182<F: Float>(t761: F, t9892: F, t2427: F, t2655: F, t152: F, t31: F, t185: F, t9288: F, t2448: F, t67: F, t758: F, t2368: F, t2505: F, t745: F, t9820: F, t9824: F, t9881: F, t9884: F, t9887: F, t9890: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9894, t9896, t9897, t9898, t9900, t9901, t9903, t9905) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk832::<F>(t761, t9892, t2427, t2655, t152, t31, t185, t9288, t2448, t67, t758, t2368, t2505, t745);
        let (t9907, t9908) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk833::<F>(t761, t9905, t9820, t9824, t9881, t9884, t9887, t9890, t9894, t9896, t9900, t9903);
    (t9894, t9896, t9897, t9898, t9900, t9901, t9903, t9905, t9907, t9908)
}
