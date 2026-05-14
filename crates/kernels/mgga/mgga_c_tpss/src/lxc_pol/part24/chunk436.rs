//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 436/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk436<F: Float>(t30: F, t259: F, t379: F, t1402: F, t1413: F, t1427: F, t1453: F, t1455: F, t1459: F, t1485: F, t198: F, t330: F, t995: F, t1288: F, t1289: F, t381: F, t45: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t1490 = piecewise3(t380, t1485 * t198 * t330 * t995 - t1413 + t1427 + t1453 + t1455 - t1459, t1402);
    let t1495 = piecewise3(t120, t259 * t1288 / 2.0 + t1402 * t30 / 2.0, t381 * t1289 / 2.0 + t1490 * t45 / 2.0);
    (t1490, t1495)
}
