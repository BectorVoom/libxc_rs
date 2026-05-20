//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta712 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2548;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta712<F: Float>(t3038: F, t49650: F, t1041: F, t13611: F, t248: F, t3051: F, t14137: F, t3117: F, t10413: F, t10422: F, t14125: F, t10965: F, t4571: F, t1020: F, t10508: F, t4650: F, t10962: F, t4630: F, t13961: F, t3114: F, t10957: F, t13950: F, t3048: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49771, t49799, t49801, t49808, t49810) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2548::<F>(t3038, t49650, t1041, t13611, t248, t3051, t14137, t3117, t10413, t10422, t14125, t10965, t4571);
        let (t49818, t49820, t49822, t49827, t49829) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2549::<F>(t1020, t10508, t248, t4650, t10962, t4630, t13961, t3114, t10957, t4571, t13950, t3048);
    (t49771, t49799, t49801, t49808, t49810, t49818, t49820, t49822, t49827, t49829)
}
