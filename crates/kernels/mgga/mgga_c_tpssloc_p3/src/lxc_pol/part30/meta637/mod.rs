//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta637<F: Float>(t25425: F, t82431: F, t25816: F, t25443: F, t1049: F, t7577: F, t7557: F, t82573: F, t23384: F, t25785: F, t25447: F, t1625: F, t6733: F) -> (F, F, F, F, F, F, F, F) {
        let (t88069, t88075, t88083, t88089, t88096, t88100, t88102, t88105) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2047::<F>(t25425, t82431, t25816, t25443, t1049, t7577, t7557, t82573, t23384, t25785, t25447, t1625, t6733);
    (t88069, t88075, t88083, t88089, t88096, t88100, t88102, t88105)
}
