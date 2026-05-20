//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk575;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk576;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta104<F: Float>(t1291: F, t2663: F, t2225: F, t522: F, t2221: F, t2223: F, t2516: F, t521: F, t17: F, t1287: F, t592: F, t588: F, t1365: F, t68: F, t248: F, t2691: F, t557: F, t555: F, t1361: F, t835: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3813, t3819, t3821, t3823, t3824) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk575::<F>(t1291, t2663, t2225, t522, t2221, t2223, t2516, t521);
        let (t3825, t3832, t3836, t3843, t3862, t3864, t3865) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk576::<F>(t17, t3824, t1287, t592, t588, t1365, t68, t248, t2691, t557, t555, t1361, t835);
    (t3813, t3819, t3821, t3823, t3824, t3825, t3832, t3836, t3843, t3862, t3864, t3865)
}
