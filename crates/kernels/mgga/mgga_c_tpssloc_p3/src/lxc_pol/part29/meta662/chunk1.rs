//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2203/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2203<F: Float>(t81311: F, t16065: F, t1992: F, t22897: F, t26378: F, t6914: F, t16044: F, t6976: F, t1372: F, t1799: F, t1307: F, t26331: F, t26446: F) -> (F, F, F, F, F, F) {
    let t90743 = F::cast_from(0.16449340668482264365e-1_f64) * t81311;
    let t90747 = t1992 * t22897 * t16065;
    let t90749 = t6914 * t26378;
    let t90750 = F::cast_from(0.76763589786250567036e-1_f64) * t90749;
    let t90752 = t1992 * t6976 * t16044;
    let t90754 = t1372 * t1799;
    let t90757 = t26331 * t26446 * t90754 * t1307;
    (t90743, t90747, t90750, t90752, t90754, t90757)
}
