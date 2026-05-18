//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 728/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk728<F: Float>(t1289: F, t725: F, t681: F, t150: F, t3589: F, t190: F, t1352: F, t2208: F, t2217: F, t2245: F, t2292: F, t2302: F, t2310: F, t2333: F, t2347: F, t2351: F, t3594: F) -> (F, F, F, F, F, F) {
    let t3642 = t725 * t1289;
    let t3643 = t681 * t3642;
    let t3644 = F::new(4.0) * t3643;
    let t3645 = t150 * t3589;
    let t3646 = t3645 * t190;
    let t3647 = t1352 * t725;
    let t3648 = t2351 + t2310 - t2208 - t2217 - t3594 + t2347 + t3644 - t2292 + t2302 + t2245 + t2333 + t3646 + t3647;
    (t3642, t3644, t3645, t3646, t3647, t3648)
}
