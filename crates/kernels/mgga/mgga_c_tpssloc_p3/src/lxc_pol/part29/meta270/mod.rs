//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1264;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1265;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1266;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta270<F: Float>(t1528: F, t1912: F, t259: F, t4147: F, t4268: F, t6549: F, t6565: F, t6627: F, t7481: F, t7486: F, t7490: F, t7492: F, t7511: F, t7517: F, t7538: F, t855: F, t870: F, t1530: F, t25: F, t1408: F, t1877: F, t1915: F, t2522: F, t6670: F, t7476: F, t1409: F, t3: F, t1484: F, t202: F, t193: F, t28: F) -> (F, F, F, F, F, F, F, F, F) {
        let t7540 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1264::<F>(t1528, t1912, t259, t4147, t4268, t6549, t6565, t6627, t7481, t7486, t7490, t7492, t7511, t7517, t7538, t855);
        let t7541 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1265::<F>(t7540, t870);
        let (t7545, t7552, t7573, t7634) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1266::<F>(t1530, t25, t1408, t1877, t1915, t2522, t6670, t7476, t7541, t1409, t3, t1484);
        let (t7642, t7649, t7650) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1267::<F>(t202, t7540, t1530, t1877, t193, t2522, t6670, t7634, t870, t1484, t28, t1915);
    (t7540, t7541, t7545, t7552, t7573, t7634, t7642, t7649, t7650)
}
