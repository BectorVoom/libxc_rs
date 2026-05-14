//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 277/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk277<F: Float>(t882: F, t884: F, t123: F, t881: F, t291: F, t287: F) -> (F, F, F, F, F, F) {
    let t885 = t882 * t884;
    let t886 = t123 * t885;
    let t888 = -t881 - 0.17808333333333333333e-1 * t886;
    let t890 = 0.621814e-1 * t888 * t291;
    let t891 = t287 * t287;
    let t892 = 1.0 / t891;
    (t885, t886, t888, t890, t891, t892)
}
