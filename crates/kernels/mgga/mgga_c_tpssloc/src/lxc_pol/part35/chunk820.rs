//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 820/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk820<F: Float>(t1239: F, t68: F, t2393: F, t374: F, t486: F, t485: F, t3585: F, t820: F, t10401: F, t3575: F, t3610: F, t3624: F, t3521: F, t10469: F, t466: F, t10471: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11604 = t1239 * t1239;
    let t11605 = 1.0 / t11604;
    let t11606 = t68 * t11605;
    let t11647 = t374 * t2393 * t486;
    let t11649 = t485 * t11647 / 10368.0;
    let t11668 = t820 * t3585;
    let t11677 = t3575 * t10401;
    let t11678 = t3610 * t11677;
    let t11692 = t3624 * t11677;
    let t11697 = t820 * t3521;
    let t11712 = t466 * t10469;
    let t11713 = t11712 * t10471;
    (t11604, t11605, t11606, t11647, t11649, t11668, t11678, t11692, t11697, t11712, t11713)
}
