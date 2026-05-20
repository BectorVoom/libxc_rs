//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk887;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk888;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta236<F: Float>(t5576: F, t838: F, t5631: F, t814: F, t252: F, t5611: F, t1499: F, t4280: F, t225: F, t5559: F, t5632: F, t5561: F, t2752: F, t5660: F, t5678: F, t690: F, t10216: F, t5392: F, t10277: F, t5682: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17000, t17027, t17030, t17034, t17052, t17090, t17092) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk887::<F>(t5576, t838, t5631, t814, t252, t5611, t1499, t4280, t225, t5559, t5632, t5561);
        let (t17116, t17149) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk888::<F>(t2752, t5660, t5678, t690);
        let (t17151, t17156, t17165) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk889::<F>(t10216, t5392, t10277, t5682, t690);
    (t17000, t17027, t17030, t17034, t17052, t17090, t17092, t17116, t17149, t17151, t17156, t17165)
}
