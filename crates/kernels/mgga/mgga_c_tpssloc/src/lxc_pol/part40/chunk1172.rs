//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1172/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1172<F: Float>(t111415: F, t1268: F, t12725: F, t1849: F, t19289: F, t20098: F, t2180: F, t2183: F, t2314: F, t26114: F, t26117: F, t26179: F, t28002: F, t28007: F, t30180: F, t30181: F, t30195: F, t30201: F, t30433: F, t30454: F, t4034: F, t510: F, t55943: F, t6287: F, t652: F, t7458: F, t7676: F, t8143: F, t8148: F, t8150: F, t8231: F, t8235: F, t8237: F, t96356: F) -> (F,) {
    let t111592 = -2.0 * t2314 * t30433 - 2.0 * t4034 * t30433 - 2.0 * t652 * t19289 * t2180 - 2.0 * t652 * t510 * t111415 + 2.0 * t55943 * t2183 - 2.0 * t652 * t6287 * t8143 + 4.0 * t12725 * t8235 + 2.0 * t1268 * t2180 * t20098 + 4.0 * t96356 * t2183 + 4.0 * t28002 * t8148 + 4.0 * t7676 * t30181 + 4.0 * t7676 * t30201 + 4.0 * t12725 * t8237 - 4.0 * t26179 * t8231 - 4.0 * t7458 * t30195 + 4.0 * t26114 * t8237 + 4.0 * t26117 * t8237 + 2.0 * t28007 * t8150 + 2.0 * t2314 * t30454 + 4.0 * t1268 * t30180 * t1849;
    (t111592,)
}
