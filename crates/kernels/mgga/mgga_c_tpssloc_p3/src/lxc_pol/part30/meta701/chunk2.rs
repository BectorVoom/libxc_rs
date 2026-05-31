//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2267/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2267<F: Float>(t870: F, t99042: F, t16596: F, t86721: F, t1484: F, t584: F, t86753: F, t22959: F, t16949: F, t25014: F, t1408: F, t4255: F) -> (F, F, F, F, F) {
    let t99043 = t99042 * t870;
    let t99049 = t86721 * t16596;
    let t99053 = t86753 * t584 * t1484;
    let t99055 = F::cast_from(6.0_f64) * t22959 * t99053;
    let t99056 = t25014 * t16949;
    let t99060 = t870 * t1408 * t4255;
    (t99043, t99049, t99055, t99056, t99060)
}
