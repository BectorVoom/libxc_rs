//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2872/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2872<F: Float>(t49072: F, t49240: F, t912: F, t13727: F, t14382: F, t14385: F, t49489: F, t13520: F, t14392: F, t14396: F, t49274: F, t2836: F, t2842: F, t5695: F) -> (F, F, F, F, F, F) {
    let t60033 = F::cast_from(0.2069040516770936012e4_f64) * t49240 * t49072 * t912;
    let t60035 = F::new(4.0) * t13727 * t14382;
    let t60037 = F::cast_from(0.19298375398431042081e3_f64) * t49489 * t14385;
    let t60039 = F::cast_from(0.32163958997385070134e2_f64) * t13520 * t14392;
    let t60041 = F::cast_from(0.1034520258385468006e4_f64) * t49274 * t14396;
    let t60044 = F::new(6.0) * t2842 * t5695 * t2836;
    (t60033, t60035, t60037, t60039, t60041, t60044)
}
