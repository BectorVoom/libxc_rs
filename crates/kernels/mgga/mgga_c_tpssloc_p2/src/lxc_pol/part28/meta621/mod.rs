//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1942;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta621<F: Float>(t22779: F, t26319: F, t1358: F, t26248: F, t3862: F, t7715: F, t22705: F, t22852: F, t236: F, t5286: F, t550: F, t26245: F, t80791: F, t22788: F, t5310: F, t16150: F, t6952: F, t16155: F, t26271: F, t80836: F, t1361: F, t22690: F, t22792: F, t5187: F, t16148: F, t26288: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t91300, t91303, t91305, t91310, t91312) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1942::<F>(t22779, t26319, t1358, t26248, t3862, t7715, t22705, t22852, t236, t5286, t550, t26245, t80791);
        let (t91317, t91319, t91321, t91323, t91327, t91330) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1943::<F>(t22788, t5310, t16150, t6952, t16155, t26271, t80836, t1361, t22690, t22792, t5187, t16148, t26288);
    (t91300, t91303, t91305, t91310, t91312, t91317, t91319, t91321, t91323, t91327, t91330)
}
