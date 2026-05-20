//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta419<F: Float>(t3375: F, t4857: F, t1157: F, t1164: F, t3400: F, t4883: F, t3411: F, t4884: F, t225: F, t4947: F, t4943: F, t1734: F, t3590: F) -> (F, F, F, F, F, F, F, F) {
        let (t14961, t14963, t14967, t14969, t14971, t14972, t14980, t14985) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1938::<F>(t3375, t4857, t1157, t1164, t3400, t4883, t3411, t4884, t225, t4947, t4943, t1734, t3590);
    (t14961, t14963, t14967, t14969, t14971, t14972, t14980, t14985)
}
