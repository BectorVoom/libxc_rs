//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1238;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1239;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta272<F: Float>(t1894: F, t7496: F, t6591: F, t1510: F, t815: F, t6605: F, t1499: F, t1898: F, t249: F, t1512: F, t6614: F, t1516: F, t6621: F, t6580: F, t6587: F, t6603: F, t6618: F, t7494: F, t218: F, t1527: F, t1911: F, t2718: F) -> (F, F, F, F, F, F) {
        let (t7497, t7498, t7500, t7501, t7503, t7504, t7506, t7508) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1238::<F>(t1894, t7496, t6591, t1510, t815, t6605, t1499, t1898, t249, t1512, t6614, t1516, t6621);
        let t7510 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1239::<F>(t6580, t6587, t6603, t6618, t7494, t7498, t7501, t7504, t7506, t7508);
        let (t7511, t7517) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1240::<F>(t218, t7510, t1527, t1911, t2718);
    (t7497, t7500, t7503, t7510, t7511, t7517)
}
