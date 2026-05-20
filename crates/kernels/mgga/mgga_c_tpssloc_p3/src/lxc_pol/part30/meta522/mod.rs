//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta522<F: Float>(t1388: F, t1845: F, t26162: F, t26161: F, t532: F, t7752: F, t6879: F, t1983: F, t1874: F, t26114: F, t4072: F, t89: F) -> (F, F, F, F, F, F, F, F) {
        let (t26163, t26164, t26166, t26167, t26168, t26170, t26178, t26179) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1861::<F>(t1388, t1845, t26162, t26161, t532, t7752, t6879, t1983, t1874, t26114, t4072, t89);
    (t26163, t26164, t26166, t26167, t26168, t26170, t26178, t26179)
}
