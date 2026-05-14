//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 971/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk971<F: Float>(t10383: F, t10439: F, t10287: F, t10289: F, t10292: F, t10303: F, t10306: F, t10309: F, t1317: F, t1976: F, t1981: F, t1982: F, t2049: F, t3418: F, t3423: F, t3486: F, t578: F, t619: F, t7679: F, t7682: F, t7690: F, t91: F) -> (F, F) {
    let t10440 = t10383 + t10439;
    let t10443 = t10287 * t91 - 8.0 * t10289 * t619 + 20.0 * t10292 * t1982 - 120.0 * t10303 * t7690 + 40.0 * t10306 * t1981 + 20.0 * t10309 * t1981 - 4.0 * t10440 * t578 - 4.0 * t1317 * t7679 - 8.0 * t1976 * t3486 - 4.0 * t2049 * t3418 + 40.0 * t3423 * t7682;
    (t10440, t10443)
}
