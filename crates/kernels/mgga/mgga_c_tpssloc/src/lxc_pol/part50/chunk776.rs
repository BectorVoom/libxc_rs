//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 776/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk776<F: Float>(t533: F, t8492: F, t3701: F, t1983: F, t113: F, t1869: F, t1976: F, t510: F, t574: F, t8313: F, t8315: F, t8322: F, t8324: F, t8329: F, t8439: F, t8447: F, t8451: F, t8491: F) -> (F, F, F) {
    let t8493 = t533 * t8492;
    let t8494 = t8493 * t3701;
    let t8495 = t1983 * t8494;
    let t8496 = -t113 * t8439 - 2.0 * t1869 * t1976 - t510 * t8313 + t574 * t8447 - 4.0 * t8315 - t8322 - 4.0 * t8324 - t8329 + 2.0 * t8451 + t8491 - t8495;
    (t8493, t8494, t8496)
}
