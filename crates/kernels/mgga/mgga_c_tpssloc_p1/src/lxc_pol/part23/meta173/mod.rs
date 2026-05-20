//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk790;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta173<F: Float>(t761: F, t9713: F, t177: F, t2508: F, t2512: F, t9490: F) -> (F, F, F) {
        let (t9715, t9720) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk790::<F>(t761, t9713, t177, t2508);
        let t9722 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk791::<F>(t2512, t9490, t9720);
    (t9715, t9720, t9722)
}
