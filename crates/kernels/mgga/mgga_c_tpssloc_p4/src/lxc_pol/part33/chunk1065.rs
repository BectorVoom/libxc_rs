//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1065/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1065<F: Float>(t28: F, t265: F, t504: F, t21076: F, t21999: F, t22412: F, t1409: F, t1534: F, t1649: F, t1768: F, t20217: F, t20390: F, t506: F, t52: F, t5398: F, t5669: F, t5966: F, t6279: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t22414 = piecewise3::<F>(t505, t21999 + t22412, t21076);
    let t22424 = piecewise3::<F>(t401, t21076 * t28 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t5669 * t1649 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1534 * t5966 + t265 * t20390 / F::cast_from(2.0_f64), t22414 * t52 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6279 * t1409 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1768 * t5398 - t506 * t20217 / F::cast_from(2.0_f64));
    t22424
}
