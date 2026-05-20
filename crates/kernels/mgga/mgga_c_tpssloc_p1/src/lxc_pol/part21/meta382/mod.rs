//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta382<F: Float>(t3114: F, t4630: F, t248: F, t3101: F, t4650: F, t1020: F, t10508: F, t1616: F, t122: F, t247: F) -> (F, F, F, F, F, F) {
        let (t13959, t13961, t13963, t13965, t13966, t13969) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1843::<F>(t3114, t4630, t248, t3101, t4650, t1020, t10508, t1616, t122, t247);
    (t13959, t13961, t13963, t13965, t13966, t13969)
}
