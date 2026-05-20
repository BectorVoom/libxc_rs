//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta274 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1439;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1440;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1441;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1442;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta274<F: Float>(t10470: F, t10471: F, t1013: F, t363: F, t3034: F, t6793: F, t368: F, t1022: F, t3040: F, t3131: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t10472 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1439::<F>(t10470, t10471);
        let (t10473, t10474, t10475, t10477) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1440::<F>(t1013, t363, t3034, t6793);
        let (t10478, t10479, t10480) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1441::<F>(t10477, t368, t10475, t10472);
        let t10481 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1442::<F>(t1022, t3040);
        let t10482 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1443::<F>(t3131, t360);
    (t10472, t10473, t10474, t10475, t10477, t10478, t10479, t10480, t10481, t10482)
}
