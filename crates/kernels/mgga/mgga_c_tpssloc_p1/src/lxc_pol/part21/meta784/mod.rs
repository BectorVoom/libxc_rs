//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta784 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2718;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta784<F: Float>(t54460: F, t54462: F, t39851: F, t39857: F, t54467: F, t54469: F, t54471: F, t40221: F, t40225: F, t19573: F, t588: F, t592: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t57218, t57219, t57220, t57221, t57222, t57223, t57224, t57225, t57226, t57228, t57229) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2718::<F>(t54460, t54462, t39851, t39857, t54467, t54469, t54471, t40221, t40225, t19573, t588, t592);
    (t57218, t57219, t57220, t57221, t57222, t57223, t57224, t57225, t57226, t57228, t57229)
}
