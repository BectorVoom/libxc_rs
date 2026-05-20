//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2294;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta575<F: Float>(t19482: F, t666: F, t5468: F, t9384: F, t659: F, t1444: F, t2: F, t584: F, t2341: F, t5396: F, t9212: F, t95: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19483, t19488, t19489, t19492, t19493, t19498, t19499, t19503, t19504) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2294::<F>(t19482, t666, t5468, t9384, t659, t1444, t2, t584, t2341, t5396, t9212, t95);
    (t19483, t19488, t19489, t19492, t19493, t19498, t19499, t19503, t19504)
}
