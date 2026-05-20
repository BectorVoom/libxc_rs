//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 765/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk765<F: Float>(t3584: F, t61: F, t1236: F, t225: F, t1239: F, t496: F, t68: F, t1243: F, t3534: F, t3032: F, t3502: F, t3499: F) -> (F, F, F, F, F, F) {
    let t3585 = t61 * t3584;
    let t3593 = t1236 * t225;
    let t3597 = F::new(1.0) / t1239 / t496;
    let t3598 = t68 * t3597;
    let t3604 = t3534 * t1243;
    let t3609 = t3032 * t3502;
    let t3610 = t3499 * t3609;
    (t3585, t3593, t3598, t3604, t3609, t3610)
}
