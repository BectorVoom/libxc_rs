//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1014/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1014<F: Float>(t248: F, t3521: F, t5975: F, t1227: F, t1409: F, t15701: F, t15700: F, t3578: F, t1735: F, t4729: F, t18232: F, t4900: F, t3450: F, t5398: F, t3449: F, t18237: F, t4908: F) -> (F, F, F, F, F, F) {
    let t18392 = t248 * t3521 * t5975;
    let t18393 = t1227 * t18392;
    let t18395 = t15701 * t1409;
    let t18396 = t15700 * t18395;
    let t18397 = t3578 * t18396;
    let t18400 = t1735 * t4729;
    let t18401 = t3578 * t18400;
    let t18404 = t4900 * t18232;
    let t18409 = t3450 * t5398;
    let t18410 = t3449 * t18409;
    let t18413 = t4908 * t18237;
    (t18393, t18397, t18401, t18404, t18410, t18413)
}
