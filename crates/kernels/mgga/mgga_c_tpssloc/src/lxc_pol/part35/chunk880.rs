//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 880/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk880<F: Float>(t1453: F, t5464: F, t9365: F, t4043: F, t5488: F, t1444: F, t5468: F, t9384: F, t4049: F, t5396: F, t20215: F, t95: F, t5415: F, t1449: F, t5480: F, t9398: F, tau1: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20304 = t5464 * t1453;
    let t20305 = t9365 * t20304;
    let t20308 = t4043 * t5488;
    let t20311 = t5468 * t1444;
    let t20312 = t9384 * t20311;
    let t20315 = t4049 * t5396;
    let t20318 = 3.0 * t20215;
    let t20319 = t95 * t20318;
    let t20322 = tau1 * t5415;
    let t20331 = t5480 * t1449;
    let t20332 = t9398 * t20331;
    (t20304, t20305, t20308, t20312, t20315, t20318, t20319, t20322, t20332)
}
