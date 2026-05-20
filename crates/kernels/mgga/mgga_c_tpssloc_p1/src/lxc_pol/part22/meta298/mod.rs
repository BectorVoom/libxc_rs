//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1460;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1461;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta298<F: Float>(t3117: F, t4571: F, t248: F, t3051: F, t4347: F, t1041: F, t3114: F, t4630: F, t3101: F, t4650: F, t1020: F, t10508: F, t1616: F, t122: F, t247: F, t4599: F, t3039: F, t3069: F, t4669: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13948, t13950, t13952, t13959, t13961, t13963, t13965) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1460::<F>(t3117, t4571, t248, t3051, t4347, t1041, t3114, t4630, t3101, t4650, t1020, t10508, t1616);
        let (t13966, t13969) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1461::<F>(t1020, t13965, t122, t247);
        let (t13970, t13972, t13995) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1462::<F>(t13969, t4599, t3039, t3069, t4669);
    (t13948, t13950, t13952, t13959, t13961, t13963, t13965, t13966, t13969, t13970, t13972, t13995)
}
