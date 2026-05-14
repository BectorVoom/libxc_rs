//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 417/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk417<F: Float>(t25: F, t265: F, t394: F, t1052: F, t1920: F, t1923: F, t1946: F, t1956: F, t388: F, t1914: F, t202: F, t193: F, t870: F, t1070: F, t336: F, t1918: F, t40: F, t1915: F, t28: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t1958 = 0.82246703342411321825e-2 * t1920 * t1923 + t1946 * t388 - t1052 * t1956;
    let t1962 = t202 * t1914;
    let t1964 = t193 * t1962 * t870;
    let t1965 = piecewise3(t395, t193 * t336 * t1958 * t1070, t1964);
    let t1968 = piecewise3(t115, t1918, t1965 * t40 / 2.0);
    let t1969 = t1915 * t28;
    (t1958, t1962, t1964, t1965, t1968, t1969)
}
