//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1151/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1151<F: Float>(t1982: F, t77: F, t84: F, t1981: F, t582: F, t1679: F, t619: F, t615: F, t2049: F, t1985: F, t578: F, t1993: F, t2056: F, t5532: F, t13235: F, t1689: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18347 = t77 * t84 * t1982;
    let t18350 = t1981 * t582;
    let t18351 = t1679 * t619;
    let t18356 = t77 * t615 * t619;
    let t18360 = t77 * t84 * t2049;
    let t18363 = t578 * t1985;
    let t18366 = t578 * t1993;
    let t18384 = 4.0 * t2056 * t5532;
    let t18386 = 2.0 * t13235 * t1689;
    (t18347, t18350, t18351, t18356, t18360, t18363, t18366, t18384, t18386)
}
