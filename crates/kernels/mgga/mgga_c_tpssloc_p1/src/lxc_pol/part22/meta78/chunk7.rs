//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 548/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk548<F: Float>(t25: F, t1408: F, t1409: F, t1534: F, t1642: F, t265: F, t396: F, t40: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t1647 = piecewise3::<F>(t115, t265 * t1408 / F::new(2.0) + t1534 * t25 / F::new(2.0), t396 * t1409 / F::new(2.0) + t1642 * t40 / F::new(2.0));
    let t1649 = -t1408;
    (t1647, t1649)
}
