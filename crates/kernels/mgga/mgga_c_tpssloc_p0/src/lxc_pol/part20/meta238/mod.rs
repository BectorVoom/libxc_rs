//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1338;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta238<F: Float>(t819: F, t820: F, t9661: F, t2628: F, t835: F, t812: F, t2635: F, t2690: F, t815: F, t831: F, t2617: F, t2638: F) -> (F, F, F, F, F, F, F, F) {
        let (t9663, t9666, t9667, t9668, t9670, t9671) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1338::<F>(t819, t820, t9661, t2628, t835, t812, t2635, t2690, t815);
        let (t9672, t9674) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1339::<F>(t831, t9671, t2617, t2638);
    (t9663, t9666, t9667, t9668, t9670, t9671, t9672, t9674)
}
