//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta413<F: Float>(t21036: F, t225: F, t20852: F, t252: F, t1519: F, t5611: F, t21013: F, t814: F, t20937: F, t68: F, t20217: F, t707: F, t751: F) -> (F, F, F, F, F, F) {
        let (t67344, t67392, t67405, t67429, t67441, t67463) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1231::<F>(t21036, t225, t20852, t252, t1519, t5611, t21013, t814, t20937, t68, t20217, t707, t751);
    (t67344, t67392, t67405, t67429, t67441, t67463)
}
