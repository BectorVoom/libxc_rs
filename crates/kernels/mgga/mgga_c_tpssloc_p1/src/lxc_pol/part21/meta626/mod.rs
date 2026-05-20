//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta626<F: Float>(t2531: F, t9919: F, t9467: F, t9879: F, t2374: F, t39519: F, t39503: F, t118: F, t2375: F, t2448: F, t39391: F, t761: F) -> (F, F, F, F, F, F) {
        let (t40733, t40738, t40741, t40743, t40745, t40748) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2407::<F>(t2531, t9919, t9467, t9879, t2374, t39519, t39503, t118, t2375, t2448, t39391, t761);
    (t40733, t40738, t40741, t40743, t40745, t40748)
}
