//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 812/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk812<F: Float>(t112892: F, t1902: F, t213: F, t225: F, t22986: F, t23272: F, t23035: F, t23241: F, t30663: F, t1880: F, t82124: F, t8335: F, t23237: F, t30656: F, t1888: F, t23270: F, t2742: F, t30633: F) -> (F, F, F, F, F, F) {
    let t112893 = 0.16449340668482264365e-1 * t112892;
    let t112899 = t213 * t1902 * t225;
    let t112902 = 0.6579736267392905746e-1 * t22986 * t112899 * t23272;
    let t112905 = 0.9869604401089358619e-1 * t23035 * t30663 * t23241;
    let t112915 = 0.16449340668482264365e-1 * t1880 * t82124 * t8335;
    let t112920 = 0.3289868133696452873e-1 * t1880 * t23237 * t30656;
    let t112927 = 0.3289868133696452873e-1 * t1888 * t23270 * t30633 * t2742;
    (t112893, t112902, t112905, t112915, t112920, t112927)
}
