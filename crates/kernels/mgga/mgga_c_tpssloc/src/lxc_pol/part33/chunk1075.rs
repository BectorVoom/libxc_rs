//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1075/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1075<F: Float>(t23146: F, t5593: F, t1894: F, t236: F, t5544: F, t6591: F, t23056: F, t5568: F, t5527: F, t23078: F, t1484: F, t1509: F, t232: F, t815: F, t23097: F, t1516: F, t25068: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28380 = t23146 * t5593;
    let t28383 = t1894 * t236 * t5544;
    let t28384 = t6591 * t28383;
    let t28386 = t23056 * t5568;
    let t28389 = t1894 * t236 * t5527;
    let t28390 = t23078 * t28389;
    let t28395 = t1484 * t1509 * t232;
    let t28396 = t815 * t28395;
    let t28397 = t23097 * t28396;
    let t28399 = t25068 * t1516;
    (t28380, t28383, t28384, t28386, t28389, t28390, t28395, t28396, t28397, t28399)
}
