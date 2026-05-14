//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 403/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk403<F: Float>(t1902: F, t218: F, t1894: F, t252: F, t214: F, t1880: F, t235: F, t226: F) -> (F, F, F, F, F) {
    let t1903 = t218 * t1902;
    let t1905 = t1894 * t252;
    let t1906 = t214 * t1905;
    let t1907 = t1880 * t1906;
    let t1909 = t235 * t1902;
    let t1911 = 0.82246703342411321825e-2 * t1907 + t226 * t1909;
    (t1903, t1905, t1906, t1909, t1911)
}
