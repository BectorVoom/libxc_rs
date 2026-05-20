//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1363/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1363<F: Float>(t25: F, t105830: F, t106607: F, t1409: F, t1965: F, t20217: F, t28756: F, t40: F, t5398: F, t7643: F, t105769: F, t25927: F, t105754: F, t23788: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t106617 = piecewise3::<F>(t115, t105830, t106607 * t40 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t28756 * t1409 + F::new(3.0) / F::new(2.0) * t7643 * t5398 + t1965 * t20217 / F::new(2.0));
    let t106618 = t25927 * t105769;
    let t106621 = t23788 * t105754;
    (t106617, t106618, t106621)
}
