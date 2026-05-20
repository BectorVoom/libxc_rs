//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1108;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1109;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1110;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta187<F: Float>(t103: F, t5484: F, t100: F, t104: F, t1447: F, t1450: F, t5469: F, t5472: F, t5475: F, t5481: F, t92: F, t109: F, t656: F, t2327: F, t4041: F, t5465: F, t64: F, t510: F, t40: F, t4100: F, t4102: F, t185: F, t5392: F, t2658: F, t1484: F, t4310: F, t1462: F, t4205: F, t2433: F, t5398: F, t73: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5485, t5488) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1108::<F>(t103, t5484, t100, t104, t1447, t1450, t5469, t5472, t5475, t5481, t92);
        let (t5489, t5493) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1109::<F>(t109, t5488, t656, t2327, t4041, t5465, t64);
        let t5494 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1110::<F>(t510, t5493);
        let (t5497, t5498, t5499, t5501, t5502, t5506, t5512) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1111::<F>(t40, t4100, t4102, t185, t5392, t2658, t1484, t4310, t1462, t4205, t2433, t5398, t73, zeta_threshold);
    (t5485, t5488, t5489, t5493, t5494, t5497, t5498, t5499, t5501, t5502, t5506, t5512)
}
