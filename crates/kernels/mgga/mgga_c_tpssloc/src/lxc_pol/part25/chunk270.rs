//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 270/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk270<F: Float>(t882: F, t884: F, t123: F, t881: F, t291: F, t287: F, t275: F, t276: F, t880: F) -> (F, F, F, F, F, F, F, F, F) {
    let t885 = t882 * t884;
    let t886 = t123 * t885;
    let t888 = -t881 - F::cast_from(0.17808333333333333333e-1_f64) * t886;
    let t890 = F::new(0.621814e-1) * t888 * t291;
    let t891 = t287 * t287;
    let t892 = F::new(1.0) / t891;
    let t893 = t275 * t892;
    let t894 = F::new(1.0) / t276;
    let t896 = -t880 / F::new(3.0) - t886 / F::new(3.0);
    (t885, t886, t888, t890, t891, t892, t893, t894, t896)
}
