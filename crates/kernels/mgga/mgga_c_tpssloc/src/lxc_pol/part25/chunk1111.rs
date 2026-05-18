//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1111/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1111<F: Float>(t1307: F, t6637: F, t6888: F, t81129: F, t22747: F, t22893: F, t80681: F, t154: F, t9533: F, t131: F, t3748: F, t2009: F, t9537: F) -> (F, F, F, F, F) {
    let t81132 = t6888 * t6637 * t81129 * t1307;
    let t81140 = t80681 * t22893 * t22747;
    let t81142 = t9533 * t154;
    let t81144 = t81142 * t3748 * t131;
    let t81146 = t81144 * t9537 * t2009;
    (t81132, t81140, t81142, t81144, t81146)
}
