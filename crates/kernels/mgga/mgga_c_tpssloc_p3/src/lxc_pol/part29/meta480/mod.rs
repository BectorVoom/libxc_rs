//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta480<F: Float>(t1186: F, t7295: F, t1235: F, t7284: F, t7287: F, t1240: F, t1251: F, t2122: F, t1170: F, t2121: F, t461: F, t6729: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t24630, t24633, t24634, t24637, t24638, t24639, t24645, t24646, t24649) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1819::<F>(t1186, t7295, t1235, t7284, t7287, t1240, t1251, t2122, t1170, t2121, t461, t6729);
    (t24630, t24633, t24634, t24637, t24638, t24639, t24645, t24646, t24649)
}
