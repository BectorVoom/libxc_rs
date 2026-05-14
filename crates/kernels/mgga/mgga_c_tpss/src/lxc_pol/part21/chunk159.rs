//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 159/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk159<F: Float>(t467: F, t473: F, t198: F, t330: F, t410: F, t438: F, t440: F, t259: F) -> (F, F) {
    let t475 = t467 * t473 + 1.0;
    let t476 = f64::ln(t475);
    let t479 = t198 * t330 * t476 - t410 + t438 + t440;
    let t480 = t259 < t479;
    let t481 = piecewise3(t480, t479, t259);
    (t475, t481)
}
