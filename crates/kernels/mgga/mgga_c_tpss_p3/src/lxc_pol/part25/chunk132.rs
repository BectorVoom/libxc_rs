//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 132/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk132<F: Float>(t367: F, t373: F, t198: F, t287: F, t322: F, t324: F, t330: F, t259: F) -> (F, F, F) {
    let t375 = t367 * t373 + F::cast_from(1.0_f64);
    let t376 = F::ln(t375);
    let t379 = t198 * t330 * t376 - t287 + t322 + t324;
    let t380 = t259 < t379;
    let t381 = piecewise3::<F>(t380, t379, t259);
    (t375, t381, t379)
}
