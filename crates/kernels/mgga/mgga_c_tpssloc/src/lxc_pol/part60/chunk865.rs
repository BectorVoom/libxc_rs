//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 865/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk865<F: Float>(t30714: F, t5593: F, t5575: F, t8342: F, t8344: F, t1894: F, t5544: F, t59: F, t6591: F, t5612: F, t6605: F, t6612: F, t23046: F, t5585: F, t23078: F, t5527: F) -> (F, F, F, F, F, F) {
    let t126312 = t30714 * t5593;
    let t126316 = t5575 * t8342 * t8344;
    let t126320 = t6591 * t1894 * t59 * t5544;
    let t126325 = t6605 * t6612 * t5612;
    let t126328 = t6605 * t23046 * t5585;
    let t126332 = t23078 * t1894 * t59 * t5527;
    (t126312, t126316, t126320, t126325, t126328, t126332)
}
