//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta172 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta172<F: Float>(t2631: F, t828: F, t232: F, t819: F, t820: F, t2628: F, t835: F, t812: F, t2635: F, t2690: F, t815: F, t831: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9660, t9661, t9663, t9666, t9667, t9668, t9670, t9671, t9672) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk806::<F>(t2631, t828, t232, t819, t820, t2628, t835, t812, t2635, t2690, t815, t831);
    (t9660, t9661, t9663, t9666, t9667, t9668, t9670, t9671, t9672)
}
