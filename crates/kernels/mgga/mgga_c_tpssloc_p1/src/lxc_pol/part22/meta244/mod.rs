//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1340;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1341;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1342;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1343;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1344;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1345;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1346;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta244<F: Float>(t10402: F, t3186: F, t3062: F, t820: F, t3200: F, t3051: F, t1005: F, t3082: F, t121: F, t3061: F, t1008: F, t349: F, t1011: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t10403 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1340::<F>(t10402, t3186);
        let t10408 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1341::<F>(t3062, t820);
        let t10413 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1342::<F>(t10402, t3200);
        let t10422 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1343::<F>(t3051, t820);
        let (t10436, t10457, t10468, t10469) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1344::<F>(t1005, t3082, t121, t3061, t1008);
        let t10470 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1345::<F>(t10469, t349);
        let t10471 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1346::<F>(t1011);
        let t10472 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1347::<F>(t10470, t10471);
    (t10403, t10408, t10413, t10422, t10436, t10457, t10468, t10469, t10470, t10471, t10472)
}
