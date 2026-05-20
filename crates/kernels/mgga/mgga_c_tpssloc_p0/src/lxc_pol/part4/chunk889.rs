//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 889/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk889<F: Float>(t10470: F, t10471: F, t1013: F, t363: F, t3034: F, t6793: F, t368: F, t3131: F, t360: F, t376: F, t676: F, t1023: F, t248: F) -> (F, F, F, F, F, F, F, F) {
    let t10472 = t10470 * t10471;
    let t10473 = t1013 * t1013;
    let t10474 = F::new(1.0) / t10473;
    let t10475 = t10474 * t363;
    let t10477 = F::new(1.0) / t3034 / t6793;
    let t10478 = t368 * t10477;
    let t10479 = t10475 * t10478;
    let t10480 = t10472 * t10479;
    let t10482 = t3131 * t360;
    let t10508 = t676 * t376;
    let t10510 = t248 * t10508 * t1023;
    (t10472, t10474, t10477, t10478, t10480, t10482, t10508, t10510)
}
