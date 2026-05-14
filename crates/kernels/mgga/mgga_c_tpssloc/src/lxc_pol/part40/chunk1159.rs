//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1159/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1159<F: Float>(t30424: F, t574: F, t1849: F, t8230: F, t2180: F, t6287: F, t1774: F, t510: F, t6468: F, t1268: F, t19451: F, t2181: F, t2183: F, t28002: F, t28007: F, t28030: F, t4028: F, t652: F, t7458: F, t7676: F, t8221: F, t8231: F, t8235: F, t8237: F) -> (F, F, F, F, F, F, F) {
    let t30425 = t30424 * t574;
    let t30428 = t8230 * t1849;
    let t30433 = t6287 * t2180;
    let t30444 = t1774 * t8230;
    let t30447 = t510 * t30424;
    let t30454 = t2180 * t6468;
    let t30465 = 2.0 * t1268 * t30425 + 4.0 * t1268 * t30428 + 2.0 * t1268 * t30454 - 2.0 * t19451 * t2181 + 2.0 * t19451 * t2183 - 4.0 * t2181 * t28002 - 2.0 * t2181 * t28030 + 4.0 * t2183 * t28002 + 2.0 * t2183 * t28007 - 2.0 * t30433 * t652 - 4.0 * t30444 * t652 - 2.0 * t30447 * t652 - 4.0 * t4028 * t8221 - 4.0 * t4028 * t8231 + 4.0 * t4028 * t8235 + 4.0 * t4028 * t8237 - 4.0 * t7458 * t8221 - 4.0 * t7458 * t8231 + 4.0 * t7676 * t8235 + 4.0 * t7676 * t8237;
    (t30425, t30428, t30433, t30444, t30447, t30454, t30465)
}
