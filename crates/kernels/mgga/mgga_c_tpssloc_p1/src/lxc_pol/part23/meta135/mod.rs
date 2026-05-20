//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk656;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk657;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk658;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta135<F: Float>(t218: F, t5631: F, t1527: F, t2718: F, t2728: F, t5585: F, t1510: F, t4295: F, t5612: F, t860: F, t5617: F, t235: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t4166: F, t5575: F, t812: F, t858: F, t1528: F, t259: F, t4147: F, t4268: F, t5559: F, t5561: F, t855: F, t1530: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5632, t5636, t5637, t5645, t5648, t5651, t5653, t5655) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk656::<F>(t218, t5631, t1527, t2718, t2728, t5585, t1510, t4295, t5612, t860, t5617, t235);
        let t5657 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk657::<F>(t1499, t1523, t1525, t226, t255, t4166, t5575, t5645, t5648, t5651, t5653, t5655, t812);
        let (t5658, t5660) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk658::<F>(t5657, t858, t1528, t259, t4147, t4268, t5559, t5561, t5632, t5637, t855);
        let t5664 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk659::<F>(t1530);
    (t5632, t5636, t5637, t5645, t5648, t5651, t5653, t5655, t5657, t5658, t5660, t5664)
}
