//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 151/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk151<F: Float>(t28: F, t466: F, t491: F, t477: F, t68: F, t470: F, t254: F, t193: F, t336: F, t425: F, t453: F, t455: F, t265: F, t52: F, t399: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t492 = t466 * t491;
    let t493 = t68 * t477;
    let t494 = t493 * t491;
    let t496 = t470 * t494 + 1.0;
    let t497 = 1.0 / t496;
    let t498 = t254 * t497;
    let t500 = t492 * t498 + 1.0;
    let t501 = f64::ln(t500);
    let t504 = t193 * t336 * t501 - t425 + t453 + t455;
    let t505 = t265 < t504;
    let t506 = piecewise3(t505, t504, t265);
    let t509 = piecewise3(t401, t265 * t28 / 2.0, t506 * t52 / 2.0);
    let t510 = t399 + t509;
    (t492, t493, t494, t496, t498, t500, t506, t510)
}
