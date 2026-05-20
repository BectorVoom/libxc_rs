//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1010;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1011;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1012;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1013;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1014;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1015;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta210<F: Float>(t2728: F, t5585: F, t1510: F, t4295: F, t5612: F, t860: F, t5617: F, t235: F, t5631: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t4166: F, t5575: F, t812: F, t858: F, t1528: F, t259: F, t4147: F, t4268: F, t5559: F, t5561: F, t5632: F, t5637: F, t855: F, t1530: F, t193: F, t202: F, t2378: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2665: F, t2752: F, t5527: F, t5544: F, t5596: F, t5599: F, t766: F, t870: F, t5526: F, t2770: F, t5392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5645, t5648, t5651, t5653, t5655, t5657) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1010::<F>(t2728, t5585, t1510, t4295, t5612, t860, t5617, t235, t5631, t1499, t1523, t1525, t226, t255, t4166, t5575, t812);
        let t5658 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1011::<F>(t5657, t858);
        let t5660 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1012::<F>(t1528, t259, t4147, t4268, t5559, t5561, t5632, t5637, t5658, t855);
        let t5664 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1013::<F>(t1530);
        let t5668 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1014::<F>(t193, t202, t2378, t2423, t2426, t2486, t2518, t2530, t2537, t2665, t2752, t5527, t5544, t5596, t5599, t5660, t5664, t766, t870);
        let t5669 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1015::<F>(t5526, t5668);
        let t5677 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1016::<F>(t2770, t5392);
    (t5645, t5648, t5651, t5653, t5655, t5657, t5658, t5660, t5664, t5669, t5677)
}
