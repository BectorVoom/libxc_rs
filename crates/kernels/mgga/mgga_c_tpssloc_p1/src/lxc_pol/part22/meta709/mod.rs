//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta709 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta709<F: Float>(t46196: F, t21066: F, t870: F, t5544: F, t868: F, t57947: F, t5527: F, t57960: F, t46208: F, t17116: F, t1877: F, t20947: F, t2522: F, t2523: F, t39411: F, t40714: F, t40716: F, t4303: F, t4307: F, t4314: F, t46207: F) -> (F, F, F, F, F, F) {
        let (t67105, t67112, t67127, t67132, t67133, t67134) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2305::<F>(t46196, t21066, t870, t5544, t868, t57947, t5527, t57960, t46208, t17116, t1877, t20947, t2522, t2523, t39411, t40714, t40716, t4303, t4307, t4314, t46207);
    (t67105, t67112, t67127, t67132, t67133, t67134)
}
