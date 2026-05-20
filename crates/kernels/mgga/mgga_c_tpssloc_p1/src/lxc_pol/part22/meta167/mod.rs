//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1010;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1011;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1012;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta167<F: Float>(t1222: F, t1731: F, t1744: F, t1202: F, t1743: F, t225: F, t4940: F, t68: F, t484: F, t1177: F, t4729: F, t1229: F, t3247: F, t3961: F, t4582: F, t1734: F, t486: F, t1215: F, t3508: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4957, t4959, t4961, t4964, t4965) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1010::<F>(t1222, t1731, t1744, t1202, t1743, t225, t4940, t68);
        let (t4966, t4969, t4972) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1011::<F>(t484, t4965, t1177, t4729, t1229, t3247);
        let (t4973, t4974) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1012::<F>(t3961, t4972, t4582);
        let (t4977, t4978) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1013::<F>(t1734, t486, t1215, t3508);
    (t4957, t4959, t4961, t4964, t4965, t4966, t4969, t4972, t4973, t4974, t4977, t4978)
}
