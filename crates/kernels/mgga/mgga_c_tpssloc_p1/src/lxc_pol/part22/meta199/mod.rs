//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1165;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1166;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1167;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1168;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1169;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1170;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta199<F: Float>(t1021: F, t248: F, t5867: F, t1615: F, t3131: F, t360: F) -> (F, F, F, F, F, F) {
        let t5869 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1165::<F>(t1021, t248, t5867);
        let t5872 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1166::<F>(t1615);
        let t5873 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1167::<F>(t3131, t5872);
        let t5875 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1168::<F>(t1021, t248, t5873);
        let t5878 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1169::<F>(t360, t5872);
        let t5880 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1170::<F>(t1021, t248, t5878);
    (t5869, t5872, t5873, t5875, t5878, t5880)
}
