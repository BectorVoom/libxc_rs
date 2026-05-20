//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1368;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1369;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta252<F: Float>(t360: F, t6739: F, t10471: F, t10474: F, t10470: F, t10482: F, t3127: F, t3131: F, t3215: F, t390: F, t268: F, t405: F, t6546: F, t1091: F, t2394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11048, t11058, t11059) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1368::<F>(t360, t6739, t10471, t10474, t10470);
        let (t11060, t11064, t11065) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1369::<F>(t10482, t6739, t10471, t3127, t10470);
        let (t11066, t11094, t11135, t11136, t11137) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1370::<F>(t3131, t6739, t3215, t390, t268, t405, t6546, t1091, t2394);
    (t11048, t11058, t11059, t11060, t11064, t11065, t11066, t11094, t11135, t11136, t11137)
}
