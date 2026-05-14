//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 888/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk888<F: Float>(t5: F, t128333: F, t128368: F, t112: F, t33610: F, t7685: F, t28813: F, t8607: F, t27188: F, t7468: F, t33234: F, t28045: F, t7042: F, t33358: F, t91655: F, t127107: F, t127109: F, t127111: F, t128298: F, t128300: F, t128302: F, t128303: F, t128306: F, t1849: F, t31532: F, t33601: F, t510: F, t5460: F, t6287: F, t8519: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t128370 = piecewise3(t8, 0.0, t128333 + t128368);
    let t128371 = t128370 * t112;
    let t128375 = 2.0 * t7685 * t33610;
    let t128377 = 2.0 * t8607 * t28813;
    let t128381 = 4.0 * t27188 * t7468;
    let t128383 = 4.0 * t33234 * t7468;
    let t128385 = 4.0 * t7042 * t28045;
    let t128387 = 6.0 * t91655 * t33358;
    let t128388 = -t128371 * t510 + 2.0 * t1849 * t33601 - 4.0 * t31532 * t5460 - t6287 * t8519 - t127107 - t127109 - t127111 - t128298 - t128300 - t128302 + t128303 - t128306 - t128375 - t128377 - t128381 - t128383 - t128385 - t128387;
    (t128371, t128388)
}
