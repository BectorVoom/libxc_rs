//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 542/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk542<F: Float>(t1878: F, t229: F, t805: F, t1891: F, t2230: F, t213: F, t1895: F, t202: F, t243: F, t598: F) -> (F, F, F, F, F, F, F, F) {
    let t6581 = t1878 * t229;
    let t6582 = t6581 * t805;
    let t6584 = t2230 * t1891;
    let t6585 = t6584 * t213;
    let t6586 = t6585 * t1895;
    let t6589 = 1.0 / t243 / t202;
    let t6590 = t598 * t6589;
    let t6591 = t6590 * t213;
    (t6581, t6582, t6584, t6585, t6586, t6589, t6590, t6591)
}
