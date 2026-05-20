//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1931;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1932;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta565<F: Float>(t1902: F, t5558: F, t25224: F, t7479: F, t6552: F, t23195: F, t5636: F, t6553: F, t1880: F, t5527: F, t6554: F, t23035: F, t1528: F, t17052: F, t17092: F, t1912: F, t25036: F, t25188: F, t25348: F, t259: F, t26591: F, t28265: F, t28269: F, t28274: F, t28278: F, t4147: F, t4268: F, t7517: F, t7538: F) -> (F, F, F, F, F, F, F) {
        let (t28282, t28288, t28289, t28294, t28295, t28296, t28298, t28299, t28300) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1931::<F>(t1902, t5558, t25224, t7479, t6552, t23195, t5636, t6553, t1880, t5527, t6554, t23035);
        let t28304 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1932::<F>(t1528, t17052, t17092, t1912, t25036, t25188, t25348, t259, t26591, t28265, t28269, t28274, t28278, t28282, t28289, t28296, t28300, t4147, t4268, t7517, t7538);
    (t28282, t28288, t28294, t28295, t28298, t28299, t28304)
}
