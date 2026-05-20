//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1863;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta579<F: Float>(t13193: F, t6621: F, t13198: F, t23097: F, t232: F, t46565: F, t815: F, t46644: F, t25135: F, t838: F, t2693: F, t7503: F, t25132: F, t81882: F, t6604: F, t81968: F, t13184: F, t841: F, t23083: F, t25123: F, t13191: F, t25119: F, t1878: F, t81982: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87389, t87391, t87395, t87399, t87401, t87403) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1863::<F>(t13193, t6621, t13198, t23097, t232, t46565, t815, t46644, t25135, t838, t2693, t7503);
        let (t87405, t87409, t87411, t87418, t87420) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1864::<F>(t25132, t81882, t6604, t81968, t13184, t841, t23083, t25123, t13191, t25119, t1878, t81982);
    (t87389, t87391, t87395, t87399, t87401, t87403, t87405, t87409, t87411, t87418, t87420)
}
