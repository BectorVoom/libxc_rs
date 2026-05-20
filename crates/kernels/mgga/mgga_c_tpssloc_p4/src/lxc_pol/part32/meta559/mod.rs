//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1923;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta559<F: Float>(t6420: F, t6987: F, t1825: F, t26458: F, t19743: F, t550: F, t6976: F, t1992: F, t1336: F, t22718: F, t22726: F, t26437: F, t27096: F, t28156: F, t28161: F, t28165: F, t28169: F, t28171: F, t5234: F, t544: F, t7745: F, t28155: F, t1378: F, t1375: F, t1843: F, t20029: F, t20044: F, t2016: F, t22646: F, t26184: F, t26345: F, t26477: F, t26988: F, t26993: F, t28051: F, t28053: F, t28108: F, t28111: F, t28118: F, t5215: F, t568: F, t6461: F, t6958: F, t7729: F, t7750: F) -> (F, F, F, F, F, F, F) {
        let (t28174, t28178, t28181, t28182, t28185) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1923::<F>(t6420, t6987, t1825, t26458, t19743, t550, t6976, t1992, t1336, t22718, t22726, t26437, t27096, t28156, t28161, t28165, t28169, t28171, t5234, t544, t7745);
        let (t28186, t28187, t28190) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1924::<F>(t28155, t28185, t1378, t1375, t1843, t20029, t20044, t2016, t22646, t26184, t26345, t26477, t26988, t26993, t28051, t28053, t28108, t28111, t28118, t5215, t568, t6461, t6958, t7729, t7750);
    (t28174, t28178, t28181, t28182, t28186, t28187, t28190)
}
