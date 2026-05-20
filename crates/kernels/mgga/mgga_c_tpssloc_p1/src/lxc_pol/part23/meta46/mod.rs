//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta46 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk306;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta46<F: Float>(t419: F, t409: F, t410: F, t1086: F, t407: F, t281: F, t415: F, t904: F, t241: F, t457: F, t422: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1097, t1098, t1099, t1100, t1105, t1107, t1111, t1112, t1113) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk306::<F>(t419, t409, t410, t1086, t407, t281, t415, t904, t241, t457);
        let t1118 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk307::<F>(t422);
    (t1097, t1098, t1099, t1100, t1105, t1107, t1111, t1112, t1113, t1118)
}
