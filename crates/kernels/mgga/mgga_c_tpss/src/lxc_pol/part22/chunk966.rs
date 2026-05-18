//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 966/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk966<F: Float>(t1286: F, t1980: F, t1317: F, t1982: F, t3486: F, t619: F, t2049: F, t1306: F, t1985: F, t1993: F, t3462: F, t582: F) -> (F, F, F, F, F, F, F) {
    let t10292 = t1286 * t1980;
    let t10303 = t1317 * t1982;
    let t10306 = t3486 * t619;
    let t10309 = t1317 * t2049;
    let t10314 = t1985 * t1306;
    let t10317 = t1993 * t1306;
    let t10320 = t582 * t3462;
    (t10292, t10303, t10306, t10309, t10314, t10317, t10320)
}
