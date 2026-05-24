//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 955/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk955<F: Float>(t1111: F, t12445: F, t1571: F, t3087: F, t11453: F, t4252: F, t3080: F, t1569: F, t453: F, t1141: F, t2738: F, t4270: F, t9561: F) -> (F, F, F, F, F) {
    let t12446 = t1111 * t12445;
    let t12448 = t1571 * t3087;
    let t12463 = t11453 * t4252;
    let t12465 = t3080 * t12463 / F::new(2304.0);
    let t12470 = t453 * t1569;
    let t12472 = t1141 * t12470 * t2738;
    let t12475 = t9561 * t4270;
    (t12446, t12448, t12465, t12472, t12475)
}
