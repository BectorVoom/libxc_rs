//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 904/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk904<F: Float>(t28239: F, t8607: F, t22574: F, t28830: F, t36740: F, t33610: F, t7685: F, t28813: F, t27188: F, t7468: F, t33234: F, t28045: F, t7042: F, t33358: F, t91655: F, t33363: F, t7754: F) -> (F, F, F, F, F, F, F, F, F) {
    let t128303 = t8607 * t28239;
    let t128306 = 6.0 * t22574 * t36740 * t28830;
    let t128375 = 2.0 * t7685 * t33610;
    let t128377 = 2.0 * t8607 * t28813;
    let t128381 = 4.0 * t27188 * t7468;
    let t128383 = 4.0 * t33234 * t7468;
    let t128385 = 4.0 * t7042 * t28045;
    let t128387 = 6.0 * t91655 * t33358;
    let t128393 = 2.0 * t33363 * t7754;
    (t128303, t128306, t128375, t128377, t128381, t128383, t128385, t128387, t128393)
}
