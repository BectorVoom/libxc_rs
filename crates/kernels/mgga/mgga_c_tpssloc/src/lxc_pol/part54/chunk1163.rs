//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1163/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1163<F: Float>(t113875: F, t63: F, t31860: F, t32343: F, t645: F, t8513: F, t625: F, t79: F, t641: F, t8663: F, t31: F, t31864: F, t607: F, t8308: F, t31857: F, t32344: F) -> (F, F, F, F, F, F, F) {
    let t117451 = t113875 * t63;
    let t117461 = t31860 * t8513 * t32343 * t645;
    let t117480 = t79 * t625;
    let t117483 = t8663 * t8513 * t117480 * t641;
    let t117496 = t625 * t31;
    let t117499 = t31864 * t8308 * t117496 * t607;
    let t117516 = t31857 * t32344;
    (t117451, t117461, t117480, t117483, t117496, t117499, t117516)
}
