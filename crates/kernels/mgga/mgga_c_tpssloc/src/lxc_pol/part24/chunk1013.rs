//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1013/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1013<F: Float>(t109: F, t22468: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t2332: F, t2358: F, t6530: F) -> (F, F, F) {
    let t110 = 1.0 < t109;
    let t22469 = 11.0 / 9.0 * t22468;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22472 = 2.0 / 3.0 * t22471;
    let t22473 = t63 * t2331;
    let t22474 = t22473 * t2332;
    let t22476 = t6530 * t2358;
    let t22479 = piecewise3(t110, 0.0, t22469 + t22472 + t22474 / 4.0 - t22476 / 8.0);
    (t22470, t22473, t22479)
}
