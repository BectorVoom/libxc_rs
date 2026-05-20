//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1048;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1049;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1050;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1051;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta166<F: Float>(t3: F, t3931: F, t112: F, t1395: F, t111: F, t576: F, t1401: F, t2319: F, t2363: F, t577: F, t671: F, t2218: F, t2221: F, t2225: F, t2232: F, t1406: F, t604: F, t1437: F, t645: F, t1409: F, t607: F, t25: F, t28: F, t65: F, t2219: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3932, t3938) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1048::<F>(t3, t3931, t112, t1395);
        let t3941 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1049::<F>(t111, t576);
        let (t3946, t3951) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1050::<F>(t1401, t2319, t2363, t3931, t3938, t3941, t577, t671, t2218, t2221, t2225, t2232);
        let (t3953, t3958, t3961) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1051::<F>(t1406, t604, t1437, t645, t1409, t607);
        let (t3962, t3966) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1052::<F>(t25, t28, t3961, t65, t2219, zeta_threshold);
    (t3932, t3938, t3941, t3946, t3951, t3953, t3958, t3961, t3962, t3966)
}
