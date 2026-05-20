//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta551<F: Float>(t2652: F, t9874: F, t39488: F, t761: F, t2531: F, t9919: F, t9467: F, t9879: F, t2374: F, t39519: F, t39503: F, t39391: F) -> (F, F, F, F, F, F, F) {
        let (t40722, t40732, t40733, t40738, t40741, t40743, t40748) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2051::<F>(t2652, t9874, t39488, t761, t2531, t9919, t9467, t9879, t2374, t39519, t39503, t39391);
    (t40722, t40732, t40733, t40738, t40741, t40743, t40748)
}
