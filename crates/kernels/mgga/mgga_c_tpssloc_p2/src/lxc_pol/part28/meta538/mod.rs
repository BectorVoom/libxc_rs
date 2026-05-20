//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1798;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta538<F: Float>(t1905: F, t81686: F, t9537: F, t23004: F, t23110: F, t23185: F, t23005: F, t6579: F, t23181: F, t2587: F, t81151: F, t23172: F, t133: F, t1891: F, t6601: F, t80953: F, t22816: F, t23104: F, t80967: F, t6612: F, t812: F, t836: F, t2649: F, t2690: F, t6619: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81688, t81691, t81697, t81704, t81715, t81716) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1798::<F>(t1905, t81686, t9537, t23004, t23110, t23185, t23005, t6579, t23181, t2587, t81151, t23172);
        let (t81735, t81742, t81749, t81750, t81763) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1799::<F>(t133, t1891, t6601, t80953, t22816, t23104, t80967, t6612, t812, t836, t2649, t2690, t6619);
    (t81688, t81691, t81697, t81704, t81715, t81716, t81735, t81742, t81749, t81750, t81763)
}
