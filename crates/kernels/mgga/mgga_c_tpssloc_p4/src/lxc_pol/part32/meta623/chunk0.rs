//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2030/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2030<F: Float>(t11820: F, t7339: F, t2122: F, t7319: F, t1235: F, t225: F, t461: F, t11553: F, t2121: F, t2123: F, t7288: F, t85660: F) -> (F, F, F, F, F) {
    let t86350 = t7339 * t11820;
    let t86403 = t7319 * t2122;
    let t86415 = t461 * t1235 * t225;
    let t86451 = F::cast_from(0.30461741978670859935e-2_f64) * t2121 * t11553 * t2123;
    let t86473 = t85660 * t7288;
    (t86350, t86403, t86415, t86451, t86473)
}
