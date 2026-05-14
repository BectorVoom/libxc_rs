//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 925/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk925<F: Float>(t10077: F, t1235: F, t3256: F, t339: F, t789: F, t1218: F, t230: F, t3260: F, t520: F, t512: F, t8186: F, t1206: F, t1220: F, t790: F, t3346: F, t72: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10078 = t10077 * t1235;
    let t10081 = t339 * t3256 * t789;
    let t10084 = t1218 * t1218;
    let t10085 = 1.0 / t10084;
    let t10086 = t10085 * t230;
    let t10089 = t3260 * t520;
    let t10104 = 455.0 / 1296.0 * t8186 * t512;
    let t10106 = t3260 * t1206;
    let t10117 = t339 * t1220 * t790;
    let t10120 = t3346 * t72;
    (t10078, t10081, t10085, t10086, t10089, t10104, t10106, t10117, t10120)
}
