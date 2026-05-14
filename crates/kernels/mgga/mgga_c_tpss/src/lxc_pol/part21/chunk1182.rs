//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1182/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1182<F: Float>(t5692: F, t645: F, t2056: F, t5532: F, t13235: F, t1689: F, t3499: F, t5522: F, t1753: F, t2105: F, t112: F, t234: F, t599: F, t630: F, t640: F, t2073: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18380 = t5692 * t645;
    let t18384 = 4.0 * t2056 * t5532;
    let t18386 = 2.0 * t13235 * t1689;
    let t18388 = 4.0 * t3499 * t5522;
    let t18389 = t1753 * t2105;
    let t18392 = t234 * t112;
    let t18393 = 11.0 / 9.0 * t18392;
    let t18394 = t599 * t630;
    let t18395 = t18394 * t640;
    let t18396 = 2.0 / 3.0 * t18395;
    let t18397 = t68 * t2073;
    (t18380, t18384, t18386, t18388, t18389, t18393, t18394, t18395, t18396, t18397)
}
