//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 626/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk626<F: Float>(t5: F, t1317: F, t1976: F, t1981: F, t3416: F, t3418: F, t3423: F, t3486: F, t578: F, t619: F, t91: F, t117: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t3490 = piecewise3(t8, 0.0, -4.0 * t1317 * t1976 + 20.0 * t1981 * t3423 + t3416 * t91 - 4.0 * t3418 * t619 - 4.0 * t3486 * t578);
    let t3491 = t3490 * t117;
    (t3490, t3491)
}
