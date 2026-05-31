//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 323/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk323<F: Float>(t882: F, t884: F, t123: F, t881: F, t291: F, t287: F) -> (F, F, F, F, F, F) {
    let t885 = t882 * t884;
    let t886 = t123 * t885;
    let t888 = -t881 - F::cast_from(0.17808333333333333333e-1_f64) * t886;
    let t890 = F::cast_from(0.621814e-1_f64) * t888 * t291;
    let t891 = t287 * t287;
    let t892 = F::cast_from(1.0_f64) / t891;
    (t885, t886, t888, t890, t891, t892)
}
