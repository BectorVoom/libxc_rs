//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1459/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1459<F: Float>(t111: F, t34136: F, t1437: F, t63: F, t1433: F, t117496: F, t1409: F, t31864: F, t8308: F, t32344: F, t33669: F, t33677: F) -> (F, F, F, F, F, F) {
    let t124728 = t34136 * t111;
    let t124755 = t63 * t1437;
    let t124778 = t63 * t1433;
    let t124803 = t31864 * t8308 * t117496 * t1409;
    let t124805 = t33669 * t32344;
    let t124807 = t33677 * t32344;
    (t124728, t124755, t124778, t124803, t124805, t124807)
}
