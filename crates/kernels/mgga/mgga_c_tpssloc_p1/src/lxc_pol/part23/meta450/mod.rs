//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1297;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta450<F: Float>(t20825: F, t46387: F, t67099: F, t46196: F, t5660: F, t193: F, t202: F, t2752: F, t39316: F, t39320: F, t39373: F, t39397: F, t39400: F, t39408: F, t39411: F, t40679: F, t40685: F, t40708: F, t57960: F, t46208: F, t57992: F, t1462: F, t67181: F, t16625: F, t20947: F, t2522: F, t39463: F, t39468: F, t39472: F, t39476: F, t40714: F, t40716: F, t40721: F, t40732: F, t4310: F, t4314: F, t5544: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t75854, t75855, t75856, t75862) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1297::<F>(t20825, t46387, t67099, t46196, t5660, t193, t202, t2752, t39316, t39320, t39373, t39397, t39400, t39408, t39411, t40679, t40685, t40708);
        let (t75864, t75865, t75872, t75874, t75875) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1298::<F>(t57960, t46208, t57992, t1462, t67181, t16625, t20947, t2522, t39463, t39468, t39472, t39476, t40714, t40716, t40721, t40732, t4310, t4314, t5544);
    (t75854, t75855, t75856, t75862, t75864, t75865, t75872, t75874, t75875)
}
