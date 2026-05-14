//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 899/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk899<F: Float>(t1218: F, t230: F, t3260: F, t520: F, t3267: F, t3334: F, t512: F, t8186: F, t3326: F, t1220: F, t339: F, t790: F, t3277: F, t3346: F, t72: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10084 = t1218 * t1218;
    let t10085 = 1.0 / t10084;
    let t10086 = t10085 * t230;
    let t10089 = t3260 * t520;
    let t10100 = t3267 * t3334;
    let t10104 = 455.0 / 1296.0 * t8186 * t512;
    let t10111 = t3260 * t3326;
    let t10117 = t339 * t1220 * t790;
    let t10118 = t10117 * t3277;
    let t10120 = t3346 * t72;
    let t10121 = t10120 * t240;
    (t10085, t10086, t10089, t10100, t10104, t10111, t10117, t10118, t10120, t10121)
}
