//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1181/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1181<F: Float>(t5: F, t18373: F, t117: F, t118: F, t1684: F, t1753: F, t17900: F, t17902: F, t17904: F, t17906: F, t17909: F, t17911: F, t17913: F, t17915: F, t17916: F, t18287: F, t18292: F, t18298: F, t18304: F, t2054: F, t2065: F, t2106: F, t3166: F, t485: F, t5514: F, t5692: F, t624: F, t646: F) -> (F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t18374 = piecewise3(t8, 0.0, t18373);
    let t18375 = t18374 * t117;
    let t18377 = -t118 * t18287 - t1684 * t3166 - t1753 * t2054 - 4.0 * t17916 * t646 - t18375 * t485 - 4.0 * t2065 * t5514 - 2.0 * t2106 * t5514 - 2.0 * t5692 * t624 - t17900 - t17902 + t17904 - t17906 - t17909 - t17911 - t17913 - t17915 + t18292 + t18298 - t18304;
    (t18374, t18375, t18377)
}
