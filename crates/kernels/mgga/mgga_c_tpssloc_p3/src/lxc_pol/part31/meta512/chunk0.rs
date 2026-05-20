//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1708/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1708<F: Float>(t28383: F, t6591: F, t23056: F, t5568: F, t1894: F, t236: F, t5527: F, t23078: F, t1484: F, t1509: F, t232: F, t815: F) -> (F, F, F, F, F, F) {
    let t28384 = t6591 * t28383;
    let t28386 = t23056 * t5568;
    let t28389 = t1894 * t236 * t5527;
    let t28390 = t23078 * t28389;
    let t28395 = t1484 * t1509 * t232;
    let t28396 = t815 * t28395;
    (t28384, t28386, t28389, t28390, t28395, t28396)
}
