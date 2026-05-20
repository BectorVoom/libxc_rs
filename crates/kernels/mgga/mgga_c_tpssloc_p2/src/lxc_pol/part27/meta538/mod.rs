//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta538<F: Float>(t1388: F, t1845: F, t26162: F, t26161: F, t532: F, t7752: F, t6879: F, t1983: F, t1874: F, t26114: F, t4072: F, t89: F) -> (F, F, F, F, F, F, F, F) {
        let (t26163, t26164, t26166, t26167, t26168, t26170, t26178, t26179) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1964::<F>(t1388, t1845, t26162, t26161, t532, t7752, t6879, t1983, t1874, t26114, t4072, t89);
    (t26163, t26164, t26166, t26167, t26168, t26170, t26178, t26179)
}
