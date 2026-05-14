//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 694/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk694<F: Float>(t1364: F, t256: F, t1354: F, t177: F, t737: F, t72: F, t732: F, t2342: F, t162: F, t2337: F, t1289: F, t189: F, t581: F, t190: F, t3431: F, t681: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3553 = t256 * t1364;
    let t3557 = t1354 * t177;
    let t3558 = t3557 * t737;
    let t3559 = 0.5848223622634646207e0 * t3558;
    let t3560 = t1354 * t72;
    let t3561 = t3560 * t732;
    let t3562 = 0.18311447306006545054e-3 * t3561;
    let t3563 = 0.18311447306006545054e-3 * t2342;
    let t3564 = t2337 * t162;
    let t3565 = t189 * t1289;
    let t3566 = t3565 * t581;
    let t3568 = 12.0 * t3564 * t3566;
    let t3569 = t190 * t3431;
    let t3571 = 4.0 * t681 * t3569;
    (t3553, t3557, t3559, t3560, t3562, t3563, t3564, t3565, t3566, t3568, t3569, t3571)
}
