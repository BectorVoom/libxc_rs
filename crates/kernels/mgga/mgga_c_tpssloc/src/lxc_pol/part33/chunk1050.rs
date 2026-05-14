//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1050/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1050<F: Float>(t25346: F, t26198: F, t26200: F, t26231: F, t26251: F, t26255: F, t26266: F, t26361: F, t26393: F, t26406: F, t26429: F, t26127: F, t19299: F, t33: F, t22505: F, t22510: F, t5392: F, t5398: F, t6500: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26726 = 0.16449340668482264365e-1 * t25346;
    let t26988 = 0.16449340668482264365e-1 * t26198;
    let t26993 = 0.38381794893125283518e-1 * t26200;
    let t27012 = 7.0 / 1152.0 * t26231;
    let t27019 = 7.0 / 1152.0 * t26251;
    let t27022 = 7.0 / 288.0 * t26255;
    let t27027 = 7.0 / 72.0 * t26266;
    let t27067 = 0.38381794893125283518e-1 * t26361;
    let t27082 = 0.16449340668482264365e-1 * t26393;
    let t27088 = 0.38381794893125283518e-1 * t26406;
    let t27096 = 0.38381794893125283518e-1 * t26429;
    let t27166 = 2.0 / 3.0 * t26127;
    let t27937 = t19299 * t33;
    let t27948 = 5.0 / 18.0 * t22505 * t5392 + 5.0 / 6.0 * t6500 * t5398 - t22510;
    (t26726, t26988, t26993, t27012, t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27937, t27948)
}
