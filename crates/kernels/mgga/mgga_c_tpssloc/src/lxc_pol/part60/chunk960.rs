//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 960/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk960<F: Float>(t117496: F, t1409: F, t31864: F, t8308: F, t32344: F, t33669: F, t33677: F, t1437: F, t31860: F, t32343: F, t8513: F, t117480: F, t1433: F, t8663: F) -> (F, F, F, F, F) {
    let t124803 = t31864 * t8308 * t117496 * t1409;
    let t124805 = t33669 * t32344;
    let t124807 = t33677 * t32344;
    let t124834 = t31860 * t8513 * t32343 * t1437;
    let t124838 = t8663 * t8513 * t117480 * t1433;
    (t124803, t124805, t124807, t124834, t124838)
}
