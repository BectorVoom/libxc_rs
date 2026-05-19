//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1052/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1052<F: Float>(t31104: F, t6897: F, t225: F, t567: F, t6955: F, t214: F, t1985: F, t6883: F, t8455: F, t8459: F, t1385: F, t8475: F) -> (F, F, F, F, F, F, F) {
    let t31106 = F::cast_from(0.82246703342411321825e-2_f64) * t6897 * t31104;
    let t31108 = t6955 * t225 * t567;
    let t31109 = t214 * t31108;
    let t31111 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t31109;
    let t31113 = F::cast_from(0.38381794893125283518e-1_f64) * t6883 * t8455;
    let t31115 = F::cast_from(0.38381794893125283518e-1_f64) * t6883 * t8459;
    let t31116 = t8475 * t1385;
    (t31106, t31108, t31109, t31111, t31113, t31115, t31116)
}
