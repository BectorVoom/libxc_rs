//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2420;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta737<F: Float>(t49486: F, t5695: F, t10655: F, t21253: F, t17521: F, t48763: F, t21347: F, t300: F, t961: F, t10702: F, t14395: F, t5726: F, t912: F, t17366: F, t4488: F, t959: F, t21091: F, t2940: F, t21373: F, t17930: F, t4483: F, t17564: F, t48890: F, t1068: F, t21376: F, t43637: F, t4700: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t69003, t69005, t69011, t69014, t69018) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2420::<F>(t49486, t5695, t10655, t21253, t17521, t48763, t21347, t300, t961, t10702, t14395, t5726, t912);
        let (t69021, t69023, t69025, t69027, t69030, t69031) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2421::<F>(t17366, t4488, t959, t21091, t2940, t21373, t17930, t4483, t17564, t48890, t1068, t21376, t43637, t4700, t69003, t69005, t69011, t69014, t69018);
    (t69003, t69005, t69011, t69014, t69018, t69021, t69023, t69025, t69027, t69030, t69031)
}
