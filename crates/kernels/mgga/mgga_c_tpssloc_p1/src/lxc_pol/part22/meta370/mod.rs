//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1620;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta370<F: Float>(t135: F, t5889: F, t973: F, t5893: F, t5884: F, t4593: F, t4650: F, t4582: F, t5398: F, t607: F, t4583: F, t1041: F, t13948: F, t13952: F, t13959: F, t13963: F, t13966: F, t13972: F, t2960: F, t3039: F, t5885: F, t5890: F, t5894: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17615, t17616, t17620, t17621, t17624, t17625, t17631, t17632, t17635) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1620::<F>(t135, t5889, t973, t5893, t5884, t4593, t4650, t4582, t5398, t607);
        let (t17636, t17637, t17640) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1621::<F>(t17635, t4583, t4582, t1041, t13948, t13952, t13959, t13963, t13966, t13972, t17616, t17621, t17625, t17632, t2960, t3039, t5885, t5890, t5894);
    (t17615, t17616, t17620, t17621, t17624, t17625, t17631, t17632, t17635, t17636, t17637, t17640)
}
