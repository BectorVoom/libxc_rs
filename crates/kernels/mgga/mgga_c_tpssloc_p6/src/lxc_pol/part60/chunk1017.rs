//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1017/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1017<F: Float>(t28: F, t265: F, t504: F, t128193: F, t128239: F, t128278: F, t1409: F, t33547: F, t52: F, t5398: F, t8591: F, t113: F, t128201: F, t1441: F, t7467: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t128280 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t128193);
    let t128287 = piecewise3::<F>(t401, t128239 + t128278, t128280 * t52 / F::cast_from(2.0_f64) - t33547 * t1409 - t8591 * t5398 / F::cast_from(2.0_f64));
    let t128289 = t113 * (t128201 + t128287);
    let t128296 = t1441 * t7467;
    (t128289, t128296)
}
