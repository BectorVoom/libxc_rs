//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1383;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta344<F: Float>(t2696: F, t4166: F, t849: F, t13176: F, t842: F, t1516: F, t9601: F, t1509: F, t852: F, t252: F, t4233: F, t68: F, t9971: F, t226: F, t4265: F, t814: F, t225: F, t4149: F, t4351: F, t892: F, t1543: F, t2841: F, t4389: F, t699: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13360, t13362, t13365, t13368, t13380, t13384, t13396) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1383::<F>(t2696, t4166, t849, t13176, t842, t1516, t9601, t1509, t852, t252, t4233, t68, t9971);
        let (t13397, t13433, t13463, t13515, t13520, t13550) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1384::<F>(t13396, t226, t4265, t814, t225, t4149, t4351, t892, t1543, t2841, t4389, t699);
    (t13360, t13362, t13365, t13368, t13380, t13384, t13397, t13433, t13463, t13515, t13520, t13550)
}
