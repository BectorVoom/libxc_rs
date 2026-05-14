//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1183/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1183<F: Float>(t116: F, t18679: F, t18363: F, t5791: F, t1675: F, t18331: F, t5790: F, t18646: F, t5483: F, t18645: F, t5506: F, t18305: F, t18660: F, t18360: F, t18670: F, t18366: F) -> (F, F, F, F, F, F, F, F, F) {
    let t62230 = t18679 * t116;
    let t62247 = t18363 * t5791;
    let t62250 = t1675 * t5790 * t18331;
    let t62259 = t5483 * t18646;
    let t62262 = t1675 * t18645 * t5506;
    let t62264 = t18305 * t5791;
    let t62266 = t5483 * t18660;
    let t62270 = t18670 * t18360;
    let t62273 = t18366 * t5791;
    (t62230, t62247, t62250, t62259, t62262, t62264, t62266, t62270, t62273)
}
