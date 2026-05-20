//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta691 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta691<F: Float>(t1509: F, t2631: F, t13360: F, t2703: F, t1516: F, t41052: F, t40961: F, t4261: F, t9993: F, t4166: F, t9600: F, t849: F) -> (F, F, F, F, F, F, F) {
        let (t47262, t47267, t47269, t47271, t47273, t47275, t47276) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2506::<F>(t1509, t2631, t13360, t2703, t1516, t41052, t40961, t4261, t9993, t4166, t9600, t849);
    (t47262, t47267, t47269, t47271, t47273, t47275, t47276)
}
