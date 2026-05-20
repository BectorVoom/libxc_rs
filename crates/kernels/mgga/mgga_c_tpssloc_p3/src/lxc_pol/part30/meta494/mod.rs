//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1804;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1805;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1806;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1807;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta494<F: Float>(t22960: F, t25365: F, t193: F, t1962: F, t10143: F, t25: F, t1530: F, t868: F, t606: F, t4303: F, t1408: F, t776: F, t1877: F, t1915: F, t2219: F, t22959: F, t23290: F, t25013: F, t25015: F, t25021: F, t25024: F, t25028: F, t2522: F, t25354: F, t25358: F, t6542: F, t6666: F, t6670: F, t6671: F, t7475: F, t7541: F, t7545: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25366, t25372) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1804::<F>(t22960, t25365, t193, t1962);
        let t25373 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1805::<F>(t10143, t25);
        let t25374 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1806::<F>(t1530, t868);
        let (t25375, t25377, t25381, t25385, t25392, t25397) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1807::<F>(t25373, t25374, t1530, t606, t25, t4303, t1408, t776, t868, t1877, t1915, t2219);
        let t25398 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1808::<F>(t1408, t1877, t1915, t22959, t23290, t25, t25013, t25015, t25021, t25024, t25028, t2522, t25354, t25358, t25366, t25372, t25375, t25377, t25381, t25385, t25392, t25397, t606, t6542, t6666, t6670, t6671, t7475, t7541, t7545);
    (t25366, t25372, t25373, t25374, t25375, t25377, t25381, t25385, t25392, t25397, t25398)
}
