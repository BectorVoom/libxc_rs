//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1095/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1095<F: Float>(t12450: F, t3931: F, t3081: F, t4231: F, t4245: F, t461: F, t1114: F, t11453: F, t4252: F, t3080: F, t4232: F, t1569: F, t453: F, t1141: F, t2738: F, t4270: F, t9561: F) -> (F, F, F, F, F, F, F) {
    let t12451 = t3931 * t12450;
    let t12454 = t4231 * t3081;
    let t12455 = t3931 * t12454;
    let t12458 = t461 * t4245;
    let t12459 = t12458 * t1114;
    let t12460 = t3931 * t12459;
    let t12463 = t11453 * t4252;
    let t12465 = t3080 * t12463 / 2304.0;
    let t12466 = t12458 * t4232;
    let t12467 = t3931 * t12466;
    let t12470 = t453 * t1569;
    let t12472 = t1141 * t12470 * t2738;
    let t12475 = t9561 * t4270;
    (t12451, t12455, t12460, t12465, t12467, t12472, t12475)
}
