//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta324<F: Float>(t1229: F, t676: F, t1090: F, t248: F, t1227: F, t486: F, t1216: F, t1213: F, t11552: F, t221: F, t456: F, t1197: F, t698: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11789, t11791, t11792, t11818, t11820, t11821, t11832, t11834, t11835) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1355::<F>(t1229, t676, t1090, t248, t1227, t486, t1216, t1213, t11552, t221, t456, t1197, t698);
    (t11789, t11791, t11792, t11818, t11820, t11821, t11832, t11834, t11835)
}
