//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 330/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk330<F: Float>(t25: F, t28: F, t265: F, t394: F, t504: F, t193: F, t1962: F, t870: F, t1070: F, t1958: F, t336: F, t1918: F, t40: F, t1915: F, t1877: F, t52: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t1964 = t193 * t1962 * t870;
    let t1965 = piecewise3::<F>(t395, t193 * t336 * t1958 * t1070, t1964);
    let t1968 = piecewise3::<F>(t115, t1918, t1965 * t40 / F::cast_from(2.0_f64));
    let t1969 = t1915 * t28;
    let t1971 = t1877 * t1969 / F::cast_from(2.0_f64);
    let t1972 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t1964);
    let t1975 = piecewise3::<F>(t401, t1971, t1972 * t52 / F::cast_from(2.0_f64));
    (t1965, t1968, t1972, t1975)
}
