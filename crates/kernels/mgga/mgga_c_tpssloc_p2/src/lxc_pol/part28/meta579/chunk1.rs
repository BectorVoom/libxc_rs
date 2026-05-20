//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1864/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1864<F: Float>(t25132: F, t81882: F, t6604: F, t81968: F, t13184: F, t841: F, t23083: F, t25123: F, t13191: F, t25119: F, t1878: F, t81982: F) -> (F, F, F, F, F) {
    let t87405 = t81882 * t25132;
    let t87407 = t81968 * t6604;
    let t87409 = t87407 * t841 * t13184;
    let t87411 = t23083 * t25123;
    let t87418 = t25119 * t841 * t13191;
    let t87420 = t1878 * t81982;
    (t87405, t87409, t87411, t87418, t87420)
}
