//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1469/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1469<F: Float>(t1561: F, t2860: F, t13550: F, t13563: F, t13644: F, t13602: F, t4446: F, t942: F, t1573: F, t2929: F, t13566: F, t10632: F, t1580: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14276 = t1561 * t2860;
    let t14287 = F::cast_from(0.27785333333333333334e0_f64) * t13550;
    let t14291 = F::cast_from(0.22954444444444444444e0_f64) * t13563;
    let t14321 = F::cast_from(0.13892666666666666667e0_f64) * t13644;
    let t14324 = F::cast_from(0.34431666666666666666e0_f64) * t13602;
    let t14332 = t4446 * t942;
    let t14337 = t1573 * t2929;
    let t14352 = F::cast_from(0.41203703703703703704e-2_f64) * t13563;
    let t14353 = F::cast_from(0.12361111111111111111e-1_f64) * t13566;
    let t14354 = F::cast_from(0.61805555555555555556e-2_f64) * t13602;
    let t14369 = t1580 * t10632;
    (t14276, t14287, t14291, t14321, t14324, t14332, t14337, t14352, t14353, t14354, t14369)
}
