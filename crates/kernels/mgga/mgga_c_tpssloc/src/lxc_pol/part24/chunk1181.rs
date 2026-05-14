//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1181/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1181<F: Float>(t109: F, t63: F, t9365: F, t9366: F, t2358: F, t666: F, t22473: F, t6530: F, t9411: F, t81438: F, t81440: F, t81443: F, t81445: F, t510: F, t652: F, t1983: F, t22584: F, t22591: F) -> (F, F, F) {
    let t110 = 1.0 < t109;
    let t81446 = t63 * t9365;
    let t81447 = t81446 * t9366;
    let t81449 = t666 * t2358;
    let t81450 = t22473 * t81449;
    let t81452 = t6530 * t9411;
    let t81455 = piecewise3(t110, 0.0, -t81438 - 11.0 / 3.0 * t81440 - 2.0 * t81443 + t81445 - 3.0 / 4.0 * t81447 + 3.0 / 4.0 * t81450 - t81452 / 8.0);
    let t81458 = 2.0 * t652 * t510 * t81455;
    let t81469 = 9.0 * t1983 * t22591 * t22584;
    (t81455, t81458, t81469)
}
