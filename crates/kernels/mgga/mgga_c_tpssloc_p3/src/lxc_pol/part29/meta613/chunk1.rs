//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2054/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2054<F: Float>(t7291: F, t85660: F, t24564: F, t24574: F, t11605: F, t225: F, t3597: F, t3599: F, t2122: F, t7303: F, t3590: F, t7299: F) -> (F, F, F, F, F, F) {
    let t85661 = t85660 * t7291;
    let t85669 = t24574 * t24564;
    let t85674 = t225 * t11605;
    let t85687 = t3597 * t3599;
    let t85688 = t2122 * t85687;
    let t85701 = t85660 * t7303;
    let t85707 = t7299 * t3590;
    (t85661, t85669, t85674, t85688, t85701, t85707)
}
