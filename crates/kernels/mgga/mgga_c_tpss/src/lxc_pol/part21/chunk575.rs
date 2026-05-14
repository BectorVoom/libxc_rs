//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 575/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk575<F: Float>(t2406: F, t2407: F, t2157: F, t246: F, t768: F, t806: F, t2163: F, t220: F, t229: F, t2365: F, t2370: F, t2398: F, t339: F, t783: F, t813: F) -> (F, F, F) {
    let t2408 = t2406 * t2407;
    let t2411 = t2157 * t246;
    let t2415 = t768 * t806;
    let t2425 = 2.0 * t2163 * t2411 * t339 + t220 * t229 * t2398 - t2365 * t339 * t813 - t2370 * t339 * t813 - 2.0 * t2415 * t339 * t783;
    (t2408, t2415, t2425)
}
