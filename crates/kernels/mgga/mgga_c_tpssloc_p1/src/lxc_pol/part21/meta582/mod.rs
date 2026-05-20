//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta582<F: Float>(t15979: F, t15982: F, t15984: F, t182: F, t19572: F, t16164: F, t12134: F, t12136: F, t12138: F, t12142: F, t12123: F, t12130: F, t12133: F, t12141: F, t16171: F, t9853: F, t9859: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19689, t19690, t19691, t19693, t19694, t19695, t19696, t19697, t19698, t19699) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2310::<F>(t15979, t15982, t15984, t182, t19572, t16164, t12134, t12136, t12138, t12142, t12123, t12130, t12133, t12141, t16171, t9853, t9859);
    (t19689, t19690, t19691, t19693, t19694, t19695, t19696, t19697, t19698, t19699)
}
