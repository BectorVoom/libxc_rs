//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 867/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk867<F: Float>(t1561: F, t2860: F, t13550: F, t13563: F, t13644: F, t13602: F, t4446: F, t942: F, t1573: F, t2929: F, t13566: F, t2932: F, t4471: F, t300: F, t3053: F, t4644: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14276 = t1561 * t2860;
    let t14287 = 0.27785333333333333334e0 * t13550;
    let t14291 = 0.22954444444444444444e0 * t13563;
    let t14321 = 0.13892666666666666667e0 * t13644;
    let t14324 = 0.34431666666666666666e0 * t13602;
    let t14332 = t4446 * t942;
    let t14337 = t1573 * t2929;
    let t14352 = 0.41203703703703703704e-2 * t13563;
    let t14353 = 0.12361111111111111111e-1 * t13566;
    let t14354 = 0.61805555555555555556e-2 * t13602;
    let t14409 = 0.2283111111111111111e-1 * t13566;
    let t14410 = 0.11415555555555555555e-1 * t13602;
    let t14459 = t4471 * t2932;
    let t14473 = t300 * t4446;
    let t14495 = t4644 * t3053 / 3456.0;
    (t14276, t14287, t14291, t14321, t14324, t14332, t14337, t14352, t14353, t14354, t14409, t14410, t14459, t14473, t14495)
}
