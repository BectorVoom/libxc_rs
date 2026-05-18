//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 376/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk376<F: Float>(t527: F, t790: F, t242: F, t525: F, t230: F, t522: F, t234: F, t339: F) -> (F, F, F, F) {
    let t1238 = t790 * t527;
    let t1239 = t1238 * t242;
    let t1241 = F::new(7.0) / F::new(4608.0) * t525 * t1239;
    let t1242 = t522 * t230;
    let t1244 = t339 * t1242 * t234;
    (t1239, t1241, t1242, t1244)
}
