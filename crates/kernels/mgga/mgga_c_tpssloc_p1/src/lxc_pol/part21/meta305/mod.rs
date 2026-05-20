//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta305<F: Float>(t1094: F, t3263: F, t3395: F, t3403: F, t11135: F, t11203: F, t135: F, t3477: F, t1174: F, t1176: F, t698: F) -> (F, F, F, F, F, F, F, F) {
        let (t11424, t11433, t11444, t11459, t11487, t11513, t11514, t11529) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1650::<F>(t1094, t3263, t3395, t3403, t11135, t11203, t135, t3477, t1174, t1176, t698);
    (t11424, t11433, t11444, t11459, t11487, t11513, t11514, t11529)
}
