//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 915/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk915<F: Float>(t8480: F, t967: F, t140: F, t2699: F, t925: F, t2464: F, t265: F, t2458: F, t606: F, t2645: F, t2719: F, t72: F) -> (F, F, F, F, F, F) {
    let t8481 = t967 * t8480;
    let t8483 = t140 * t2699;
    let t8484 = t925 * t8483;
    let t8491 = F::new(1.0) / t265 / t2464;
    let t8493 = F::new(1.0) / t2458 / t606;
    let t8499 = t140 * t2645;
    let t8500 = t925 * t8499;
    let t8507 = t2719 * t72;
    (t8481, t8484, t8491, t8493, t8500, t8507)
}
