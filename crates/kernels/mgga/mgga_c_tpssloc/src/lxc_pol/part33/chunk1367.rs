//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1367/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1367<F: Float>(t28: F, t265: F, t504: F, t106667: F, t106716: F, t106606: F, t1409: F, t1972: F, t20217: F, t28803: F, t52: F, t5398: F, t7664: F, t1441: F, t5493: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t106717 = t106667 + t106716;
    let t106718 = piecewise3::<F>(t505, F::new(0.0), t106606);
    let t106728 = piecewise3::<F>(t401, t106717, t106718 * t52 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t28803 * t1409 - F::new(3.0) / F::new(2.0) * t7664 * t5398 - t1972 * t20217 / F::new(2.0));
    let t106731 = t1441 * t5493;
    (t106728, t106731)
}
