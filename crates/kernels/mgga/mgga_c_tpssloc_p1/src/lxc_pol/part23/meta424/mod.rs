//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1252;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta424<F: Float>(t10480: F, t21391: F, t248: F, t3101: F, t1041: F, t10457: F, t21118: F, t1020: F, t21595: F, t14511: F, t17655: F, t10883: F, t21403: F, t21130: F, t42592: F, t21594: F, t376: F, t10422: F, t21519: F, t3070: F, t135: F, t21561: F, t973: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t70227, t70239, t70346, t70351, t70363) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1252::<F>(t10480, t21391, t248, t3101, t1041, t10457, t21118, t1020, t21595, t14511, t17655, t10883, t21403);
        let (t70389, t70391, t70404, t70497) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1253::<F>(t1041, t21130, t248, t42592, t21594, t376, t10422, t21519, t3070, t135, t21561, t973);
    (t70227, t70239, t70346, t70351, t70363, t70389, t70391, t70404, t70497)
}
