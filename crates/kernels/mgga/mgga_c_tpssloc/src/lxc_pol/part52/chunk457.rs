//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 457/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk457<F: Float>(t28: F, t265: F, t504: F, t1877: F, t1969: F, t1964: F, t52: F, t1968: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t1971 = t1877 * t1969 / 2.0;
    let t1972 = piecewise3(t505, 0.0, t1964);
    let t1975 = piecewise3(t401, t1971, t1972 * t52 / 2.0);
    let t1976 = t1968 + t1975;
    (t1971, t1972, t1976)
}
