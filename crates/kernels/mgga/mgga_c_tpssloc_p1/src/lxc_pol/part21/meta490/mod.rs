//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2096;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta490<F: Float>(t828: F, t9975: F, t16815: F, t16758: F, t4182: F, t2732: F, t5617: F, t829: F, t1499: F, t4290: F, t4166: F, t4177: F, t120: F, t5584: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2096::<F>(t828, t9975, t16815, t16758, t4182, t2732, t5617, t829, t1499, t4290, t4166, t4177);
        let t16839 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2097::<F>(t120, t5584);
    (t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836, t16839)
}
