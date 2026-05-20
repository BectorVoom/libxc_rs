//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1887/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1887<F: Float>(t1408: F, t2749: F, t13191: F, t25014: F, t13196: F, t13471: F, t25: F, t25373: F, t57921: F, t1530: F, t2249: F, t16596: F, t81547: F) -> (F, F, F, F, F, F, F) {
    let t87961 = t1408 * t2749;
    let t87978 = t25014 * t13191;
    let t87981 = t25014 * t13196;
    let t87984 = t25 * t13471;
    let t87988 = t25373 * t57921;
    let t87994 = t2249 * t1530;
    let t87998 = t81547 * t16596;
    (t87961, t87978, t87981, t87984, t87988, t87994, t87998)
}
