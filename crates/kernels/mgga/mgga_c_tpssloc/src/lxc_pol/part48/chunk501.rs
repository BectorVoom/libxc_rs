//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 501/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk501<F: Float>(t6579: F, t1878: F, t229: F, t805: F, t1891: F, t2230: F, t213: F, t1895: F, t202: F, t243: F, t598: F, t1894: F, t236: F, t776: F, t2229: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6580 = 7.0 / 288.0 * t6579;
    let t6581 = t1878 * t229;
    let t6582 = t6581 * t805;
    let t6584 = t2230 * t1891;
    let t6585 = t6584 * t213;
    let t6586 = t6585 * t1895;
    let t6587 = 0.14130464632949136799e-2 * t6586;
    let t6589 = 1.0 / t243 / t202;
    let t6590 = t598 * t6589;
    let t6591 = t6590 * t213;
    let t6593 = t1894 * t236 * t776;
    let t6594 = t6591 * t6593;
    let t6597 = 1.0 / t61 / t2229;
    (t6580, t6581, t6582, t6584, t6585, t6586, t6587, t6589, t6590, t6591, t6593, t6594, t6597)
}
