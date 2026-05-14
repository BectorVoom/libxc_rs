//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 222/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk222<F: Float>(t177: F, t713: F, t662: F, t664: F, t668: F, t673: F) -> (F, F) {
    let t714 = t177 * t713;
    let t719 = -0.86308333333333333334e0 * t662 - 0.301925e0 * t664 - 0.5501625e-1 * t668 - 0.82785e-1 * t673;
    (t714, t719)
}
