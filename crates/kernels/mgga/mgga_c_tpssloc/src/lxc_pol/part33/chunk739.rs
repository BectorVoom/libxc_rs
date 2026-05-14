//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 739/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk739<F: Float>(t25: F, t28: F, t265: F, t504: F, t1409: F, t1965: F, t40: F, t7552: F, t7643: F, t1484: F, t1915: F, t1530: F, t1649: F, t1877: F, t2522: F, t6670: F, t7541: F, t7642: F, t1972: F, t52: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t7648 = piecewise3(t115, t7552, t1965 * t1409 / 2.0 + t7643 * t40 / 2.0);
    let t7649 = t28 * t1484;
    let t7650 = t1915 * t7649;
    let t7656 = t28 * t1530;
    let t7663 = 3.0 / 2.0 * t2522 * t7650 + t1877 * t7541 * t28 / 2.0 - t1877 * t6670 * t7656 / 2.0 + t1877 * t1915 * t1649 / 2.0;
    let t7664 = piecewise3(t505, 0.0, t7642);
    let t7669 = piecewise3(t401, t7663, -t1972 * t1409 / 2.0 + t7664 * t52 / 2.0);
    (t7648, t7649, t7656, t7664, t7669)
}
