//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2194/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2194<F: Float>(t1484: F, t2249: F, t4119: F, t606: F, t1408: F, t2749: F, t10143: F, t7540: F, t13191: F, t25014: F, t13196: F, t13471: F, t25: F) -> (F, F, F, F, F, F, F) {
    let t87953 = t2249 * t1484;
    let t87957 = t606 * t4119;
    let t87961 = t1408 * t2749;
    let t87975 = t7540 * t10143;
    let t87978 = t25014 * t13191;
    let t87981 = t25014 * t13196;
    let t87984 = t25 * t13471;
    (t87953, t87957, t87961, t87975, t87978, t87981, t87984)
}
