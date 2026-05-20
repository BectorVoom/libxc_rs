//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1215;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1216;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1217;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta404<F: Float>(t3131: F, t5866: F, t3199: F, t61734: F, t3185: F, t2394: F, t5972: F, t5980: F, t5976: F) -> (F, F, F, F, F, F) {
        let (t62840, t63004, t63183, t63332) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1215::<F>(t3131, t5866, t3199, t61734, t3185, t2394, t5972);
        let t63334 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1216::<F>(t2394, t5980);
        let t63361 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1217::<F>(t2394, t5976);
    (t62840, t63004, t63183, t63332, t63334, t63361)
}
