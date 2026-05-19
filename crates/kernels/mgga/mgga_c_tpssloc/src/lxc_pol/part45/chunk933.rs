//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 933/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk933<F: Float>(t30642: F, t6562: F, t794: F, t1902: F, t213: F, t225: F, t22986: F, t23272: F, t23035: F, t23241: F, t30663: F, t1880: F, t82124: F, t8335: F) -> (F, F, F, F) {
    let t112892 = t6562 * t794 * t30642;
    let t112893 = F::cast_from(0.16449340668482264365e-1_f64) * t112892;
    let t112899 = t213 * t1902 * t225;
    let t112902 = F::cast_from(0.6579736267392905746e-1_f64) * t22986 * t112899 * t23272;
    let t112905 = F::cast_from(0.9869604401089358619e-1_f64) * t23035 * t30663 * t23241;
    let t112915 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t82124 * t8335;
    (t112893, t112902, t112905, t112915)
}
