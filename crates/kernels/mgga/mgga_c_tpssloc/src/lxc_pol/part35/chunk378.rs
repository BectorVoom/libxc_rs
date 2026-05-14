//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 378/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk378<F: Float>(t28: F, t265: F, t504: F, t1256: F, t1534: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F, t1763: F, t193: F, t336: F, t1409: F, t1649: F, t506: F, t52: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t1768 = piecewise3(t505, t1256 * t1763 * t193 * t336 - t1659 + t1673 + t1699 + t1701 - t1705, t1534);
    let t1773 = piecewise3(t401, t1534 * t28 / 2.0 + t265 * t1649 / 2.0, -t506 * t1409 / 2.0 + t1768 * t52 / 2.0);
    (t1768, t1773)
}
