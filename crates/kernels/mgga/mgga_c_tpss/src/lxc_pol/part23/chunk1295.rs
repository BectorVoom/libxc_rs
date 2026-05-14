//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1295/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1295<F: Float>(t19590: F, t61873: F, t18394: F, t3532: F, t1333: F, t2074: F, t61877: F, t640: F, t18397: F, t2100: F, t13215: F, t5527: F, t61869: F, t61874: F, t61876: F, t65437: F, t65440: F) -> (F,) {
    let t65442 = t61873 * t19590;
    let t65443 = 4.0 / 3.0 * t65442;
    let t65444 = t18394 * t3532;
    let t65445 = 2.0 / 3.0 * t65444;
    let t65446 = t1333 * t2074;
    let t65447 = t61877 * t65446;
    let t65449 = t3532 * t640;
    let t65450 = t18397 * t65449;
    let t65452 = t1333 * t2100;
    let t65453 = t18397 * t65452;
    let t65455 = t5527 * t13215;
    let t65457 = -t61869 - t65437 - 2.0 / 3.0 * t61874 + t61876 / 3.0 - 11.0 / 9.0 * t65440 - t65443 + t65445 - 3.0 / 4.0 * t65447 + t65450 / 2.0 + t65453 / 4.0 - t65455 / 8.0;
    (t65457,)
}
