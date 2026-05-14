//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 213/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk213<F: Float>(t128: F, t72: F, t661: F, t3: F, t66: F, t124: F) -> (F, F, F, F) {
    let t666 = f64::sqrt(t128);
    let t667 = t666 * t72;
    let t668 = t667 * t661;
    let t671 = 1.0 / t66 / t3;
    let t672 = t124 * t671;
    (t667, t668, t671, t672)
}
