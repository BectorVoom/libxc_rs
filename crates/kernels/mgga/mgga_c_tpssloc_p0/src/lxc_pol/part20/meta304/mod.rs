//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1547;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta304<F: Float>(t11191: F, t3315: F, t11190: F, t11135: F, t1102: F, t3270: F, t3279: F, t3287: F, t10292: F, t281: F, t415: F, t1113: F, t11163: F, t136: F, t11172: F, t1114: F, t2403: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11192, t11194, t11195, t11197, t11200, t11203, t11204, t11205) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1547::<F>(t11191, t3315, t11190, t11135, t1102, t3270, t3279, t3287, t10292, t281, t415, t1113, t11163);
        let (t11206, t11208, t11209, t11211) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1548::<F>(t11205, t136, t1113, t11172, t1114, t2403);
    (t11192, t11194, t11195, t11197, t11200, t11203, t11204, t11205, t11206, t11208, t11209, t11211)
}
