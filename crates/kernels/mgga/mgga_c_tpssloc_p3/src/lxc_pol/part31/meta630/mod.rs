//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1889;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta630<F: Float>(t19986: F, t22833: F, t5303: F, t91100: F, t1339: F, t550: F, t56812: F, t6936: F, t12289: F, t1351: F, t57342: F, t20473: F, t3788: F, t19930: F, t6952: F, t1831: F, t91191: F, t26257: F, t5314: F, t28100: F, t80853: F, t80855: F, t22788: F, t6431: F, t6427: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97320, t97322, t97326, t97333, t97337) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1889::<F>(t19986, t22833, t5303, t91100, t1339, t550, t56812, t6936, t12289, t1351, t57342, t20473, t3788);
        let (t97340, t97342, t97344, t97347, t97352, t97354) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1890::<F>(t19930, t6952, t1831, t91191, t26257, t5314, t28100, t80853, t80855, t22788, t6431, t6427);
    (t97320, t97322, t97326, t97333, t97337, t97340, t97342, t97344, t97347, t97352, t97354)
}
