//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta663<F: Float>(t14725: F, t9288: F, t136: F, t3297: F, t14748: F, t2250: F, t1113: F, t14735: F, t2244: F, t4728: F, t9258: F, t43768: F, t43770: F, t43777: F, t50846: F, t50848: F, t50851: F, t50854: F, t50859: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50861, t50863, t50865, t50867, t50869, t50871, t50873, t50875, t50877) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2484::<F>(t14725, t9288, t136, t3297, t14748, t2250, t1113, t14735, t2244, t4728, t9258, t43768, t43770, t43777, t50846, t50848, t50851, t50854, t50859);
    (t50861, t50863, t50865, t50867, t50869, t50871, t50873, t50875, t50877)
}
