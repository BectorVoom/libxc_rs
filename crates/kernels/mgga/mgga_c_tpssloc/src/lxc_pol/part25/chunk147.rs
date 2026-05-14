//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 147/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk147<F: Float>(t407: F, t410: F, t413: F, t417: F, t440: F, t300: F, t425: F, t427: F, t436: F, t338: F, t51: F) -> (F, F, F, F, F, F) {
    let t445 = 0.51785e1 * t410 + 0.905775e0 * t407 + 0.1100325e0 * t413 + 0.1241775e0 * t417;
    let t448 = 1.0 + 0.29608749977793437516e2 / t445;
    let t449 = f64::ln(t448);
    let t450 = t440 * t449;
    let t453 = t300 * (-0.310907e-1 * t427 * t436 + t425 - 0.19751673498613801407e-1 * t450);
    let t455 = 0.19751673498613801407e-1 * t300 * t450;
    let t456 = t51 * t338;
    (t445, t448, t449, t453, t455, t456)
}
