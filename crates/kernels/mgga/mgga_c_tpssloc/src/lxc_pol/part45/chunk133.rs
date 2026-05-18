//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 133/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk133<F: Float>(t25: F, t28: F, t382: F, t388: F, t193: F, t293: F, t328: F, t330: F, t336: F, t265: F, t40: F, t52: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t390 = t382 * t388 + F::new(1.0);
    let t391 = f64::ln(t390);
    let t394 = t193 * t336 * t391 - t293 + t328 + t330;
    let t395 = t265 < t394;
    let t396 = piecewise3::<f64>(t395, t394, t265);
    let t399 = piecewise3::<f64>(t115, t265 * t25 / F::new(2.0), t396 * t40 / F::new(2.0));
    let t401 = rho1 <= dens_threshold || t29;
    let t404 = F::new(1.0) / t52;
    let t405 = pow_1_3::<f64>(t404);
    (t390, t396, t399, t404, t405)
}
