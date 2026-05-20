//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 148/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk148<F: Float>(t28: F, t492: F, t498: F, t193: F, t336: F, t425: F, t453: F, t455: F, t265: F, t52: F, t399: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t500 = t492 * t498 + F::new(1.0);
    let t501 = F::ln(t500);
    let t504 = t193 * t336 * t501 - t425 + t453 + t455;
    let t505 = t265 < t504;
    let t506 = piecewise3::<F>(t505, t504, t265);
    let t509 = piecewise3::<F>(t401, t265 * t28 / F::new(2.0), t506 * t52 / F::new(2.0));
    let t510 = t399 + t509;
    (t500, t506, t510, t504)
}
