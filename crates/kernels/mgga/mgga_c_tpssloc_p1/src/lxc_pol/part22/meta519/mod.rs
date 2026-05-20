//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta519<F: Float>(t15659: F, t19056: F, t4582: F, t1735: F, t1653: F, t6225: F, t3578: F, t6230: F, t5975: F, t1734: F, t6224: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22270, t22271, t22274, t22275, t22279, t22280, t22283, t22284, t22287, t22288, t22298) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1985::<F>(t15659, t19056, t4582, t1735, t1653, t6225, t3578, t6230, t5975, t1734, t6224);
    (t22270, t22271, t22274, t22275, t22279, t22280, t22283, t22284, t22287, t22288, t22298)
}
