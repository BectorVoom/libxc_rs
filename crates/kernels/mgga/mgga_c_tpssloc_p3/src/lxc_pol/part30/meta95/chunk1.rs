//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 619/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk619<F: Float>(t25: F, t265: F, t504: F, t1918: F, t1965: F, t40: F, t1915: F, t28: F, t1877: F, t1964: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t505 = t265 < t504;
    let t1968 = piecewise3::<F>(t115, t1918, t1965 * t40 / F::cast_from(2.0_f64));
    let t1969 = t1915 * t28;
    let t1971 = t1877 * t1969 / F::cast_from(2.0_f64);
    let t1972 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t1964);
    (t1968, t1971, t1972)
}
