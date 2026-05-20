//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1205;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1206;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta398<F: Float>(t2693: F, t5576: F, t2627: F, t5631: F, t10143: F, t5660: F, t2394: F, t5678: F, t5682: F, t5686: F) -> (F, F, F, F, F, F) {
        let (t59288, t59355, t59564, t59657) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1205::<F>(t2693, t5576, t2627, t5631, t10143, t5660, t2394, t5678);
        let t59688 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1206::<F>(t2394, t5682);
        let t59694 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1207::<F>(t2394, t5686);
    (t59288, t59355, t59564, t59657, t59688, t59694)
}
