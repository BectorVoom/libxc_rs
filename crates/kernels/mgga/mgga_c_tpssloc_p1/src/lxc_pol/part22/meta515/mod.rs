//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta515<F: Float>(t1653: F, t6219: F, t3578: F, t1735: F, t5971: F, t11668: F, t5979: F, t1730: F, t6164: F, t2130: F, t47: F, t479: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22153, t22154, t22157, t22158, t22161, t22162, t22169, t22173, t22174) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1979::<F>(t1653, t6219, t3578, t1735, t5971, t11668, t5979, t1730, t6164, t2130, t47, t479);
    (t22153, t22154, t22157, t22158, t22161, t22162, t22169, t22173, t22174)
}
