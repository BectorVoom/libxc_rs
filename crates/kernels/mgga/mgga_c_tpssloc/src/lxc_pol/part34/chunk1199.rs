//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1199/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1199<F: Float>(t1992: F, t550: F, t6976: F, t74941: F, t22897: F, t3792: F, t74949: F, t20632: F, t1799: F, t6637: F, t6888: F, t97126: F) -> (F, F, F, F, F) {
    let t107281 = t1992 * t6976 * t74941 * t550;
    let t107303 = t1992 * t22897 * t74941 * t3792;
    let t107320 = t1992 * t6976 * t74949 * t550;
    let t107326 = t1992 * t6976 * t20632;
    let t107331 = t6888 * t6637 * t97126 * t1799;
    (t107281, t107303, t107320, t107326, t107331)
}
