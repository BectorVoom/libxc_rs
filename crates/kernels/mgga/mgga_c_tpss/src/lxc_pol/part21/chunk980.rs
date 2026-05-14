//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 980/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk980<F: Float>(t3431: F, t725: F, t681: F, t2112: F, t3642: F, t774: F, t8305: F, t1364: F, t782: F, t2177: F, t2174: F, t1378: F, t2162: F, t750: F, t125: F, t3664: F) -> (F, F, F, F, F, F, F, F) {
    let t10564 = t725 * t3431;
    let t10566 = 8.0 * t681 * t10564;
    let t10568 = 8.0 * t2112 * t3642;
    let t10572 = t8305 * t774;
    let t10573 = t1364 * t782;
    let t10574 = t10573 * t2177;
    let t10575 = t10572 * t10574;
    let t10578 = t2174 * t774;
    let t10579 = t1378 * t782;
    let t10581 = t10578 * t10579 * t2177;
    let t10584 = t1378 * t2162;
    let t10585 = t782 * t750;
    let t10587 = t10578 * t10584 * t10585;
    let t10590 = t125 * t3664;
    (t10566, t10568, t10575, t10579, t10581, t10584, t10587, t10590)
}
