//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1164/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1164<F: Float>(t31868: F, t32344: F, t240: F, t8307: F, t8513: F, t8663: F, t111: F, t32348: F, t112: F, t32392: F, t8843: F, t25: F, t25353: F, t606: F, t7540: F, t1408: F, t6665: F) -> (F, F, F, F, F, F, F, F) {
    let t117518 = t31868 * t32344;
    let t117527 = 55.0 / 81.0 * t8663 * t8513 * t8307 * t240;
    let t117533 = t32348 * t111;
    let t117672 = t32392 * t112;
    let t117687 = t8843 * t111;
    let t118387 = t25 * t25353;
    let t118393 = t606 * t7540;
    let t118410 = t1408 * t6665;
    (t117518, t117527, t117533, t117672, t117687, t118387, t118393, t118410)
}
