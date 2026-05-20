//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1600;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1601;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta325<F: Float>(t1090: F, t3509: F, t3578: F, t1216: F, t3252: F, t3248: F, t11642: F, t11644: F, t11649: F, t11652: F, t11655: F, t11662: F, t11665: F, t11670: F, t11674: F, t11678: F, t1227: F, t3496: F, t3506: F, t3536: F, t3577: F, t3580: F, t11677: F, t3624: F, t3516: F, t3521: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11679, t11680, t11683, t11684, t11687, t11688, t11691) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1600::<F>(t1090, t3509, t3578, t1216, t3252, t3248, t11642, t11644, t11649, t11652, t11655, t11662, t11665, t11670, t11674, t11678, t1227, t3496, t3506, t3536, t3577, t3580);
        let t11692 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1601::<F>(t11677, t3624);
        let (t11693, t11694, t11697) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1602::<F>(t1090, t3516, t3578, t3521, t820);
    (t11679, t11680, t11683, t11684, t11687, t11688, t11691, t11692, t11693, t11694, t11697)
}
