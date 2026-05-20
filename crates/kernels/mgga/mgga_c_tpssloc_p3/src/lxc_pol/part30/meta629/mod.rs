//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2034;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta629<F: Float>(t25064: F, t81788: F, t25135: F, t838: F, t2693: F, t7503: F, t25132: F, t81882: F, t6604: F, t81968: F, t23083: F, t25123: F, t1878: F, t81982: F, t25120: F, t81962: F, t7500: F, t81911: F, t22690: F, t23122: F, t4119: F, t841: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87387, t87402, t87403, t87405, t87407, t87411) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2034::<F>(t25064, t81788, t25135, t838, t2693, t7503, t25132, t81882, t6604, t81968, t23083, t25123);
        let (t87412, t87420, t87426, t87432, t87443) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2035::<F>(t87411, t1878, t81982, t25120, t6604, t81962, t7500, t81911, t22690, t23122, t4119, t841);
    (t87387, t87402, t87403, t87405, t87407, t87412, t87420, t87426, t87432, t87443)
}
