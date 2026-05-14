//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 350/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk350<F: Float>(t1893: F, t1895: F, t235: F, t59: F, t226: F, t249: F, t1894: F, t252: F, t214: F, t1880: F, t335: F, t371: F, t191: F, t513: F, t192: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1896 = t1893 * t1895;
    let t1898 = t235 * t59;
    let t1899 = t226 * t1898;
    let t1900 = t1899 * t249;
    let t1905 = t1894 * t252;
    let t1906 = t214 * t1905;
    let t1907 = t1880 * t1906;
    let t1932 = 1.0 / t371 / t335;
    let t1982 = t513 * t191;
    let t1983 = t1982 * t192;
    (t1896, t1898, t1899, t1900, t1905, t1906, t1907, t1932, t1982, t1983)
}
