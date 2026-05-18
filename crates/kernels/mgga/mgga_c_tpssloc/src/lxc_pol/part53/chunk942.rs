//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 942/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk942<F: Float>(t23185: F, t31333: F, t82074: F, t31316: F, t6547: F, t23168: F, t31378: F, t22893: F, t23164: F, t31377: F, t31390: F, t23030: F, t31381: F) -> (F, F, F, F, F, F) {
    let t114613 = t23185 * t82074 * t31333;
    let t114615 = t6547 * t31316;
    let t114659 = t23168 * t31378;
    let t114666 = t23164 * t22893 * t31377;
    let t114670 = t6547 * t31390;
    let t114672 = t23030 * t31381;
    (t114613, t114615, t114659, t114666, t114670, t114672)
}
