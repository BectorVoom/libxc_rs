//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta191 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1128;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1129;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1130;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1131;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta191<F: Float>(t2701: F, t5527: F, t820: F, t5544: F, t847: F, t1512: F, t1516: F, t249: F, t2571: F, t2602: F, t2630: F, t2643: F, t2695: F, t4152: F, t4167: F, t4170: F, t4172: F, t4187: F, t4253: F, t5568: F, t5572: F, t5576: F, t5587: F, t5593: F, t5614: F, t5619: F, t787: F, t817: F, t843: F, t218: F, t1527: F, t2718: F, t2728: F, t5585: F, t1510: F, t4295: F, t5612: F, t860: F, t5617: F, t235: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t4166: F, t5575: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5624 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1128::<F>(t2701, t5527, t820);
        let t5628 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1129::<F>(t5544, t820, t847);
        let t5631 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1130::<F>(t1512, t1516, t249, t2571, t2602, t2630, t2643, t2695, t4152, t4167, t4170, t4172, t4187, t4253, t5568, t5572, t5576, t5587, t5593, t5614, t5619, t5624, t5628, t787, t817, t843);
        let (t5632, t5636, t5637) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1131::<F>(t218, t5631, t1527, t2718);
        let (t5645, t5648, t5651, t5653, t5655, t5657) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1132::<F>(t2728, t5585, t1510, t4295, t5612, t860, t5617, t235, t5631, t1499, t1523, t1525, t226, t255, t4166, t5575, t812);
    (t5624, t5628, t5631, t5632, t5636, t5637, t5645, t5648, t5651, t5653, t5655, t5657)
}
