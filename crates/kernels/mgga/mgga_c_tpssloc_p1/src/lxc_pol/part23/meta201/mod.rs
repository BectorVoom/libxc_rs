//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta201<F: Float>(t1229: F, t676: F, t486: F, t11552: F, t221: F, t456: F, t1176: F, t3242: F, t10471: F, t11715: F, t11712: F, t11721: F, t6739: F) -> (F, F, F, F, F, F, F, F) {
        let (t11789, t11818, t11832, t11834, t11848, t11880, t11881, t11883) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk842::<F>(t1229, t676, t486, t11552, t221, t456, t1176, t3242, t10471, t11715, t11712, t11721, t6739);
    (t11789, t11818, t11832, t11834, t11848, t11880, t11881, t11883)
}
