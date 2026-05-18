//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1125/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1125<F: Float>(t1111: F, t12445: F, t1571: F, t3087: F, t3074: F, t4231: F, t3931: F, t3081: F, t4245: F, t461: F, t1114: F, t11453: F, t4252: F) -> (F, F, F, F, F, F, F) {
    let t12446 = t1111 * t12445;
    let t12448 = t1571 * t3087;
    let t12450 = t4231 * t3074;
    let t12451 = t3931 * t12450;
    let t12454 = t4231 * t3081;
    let t12455 = t3931 * t12454;
    let t12458 = t461 * t4245;
    let t12459 = t12458 * t1114;
    let t12460 = t3931 * t12459;
    let t12463 = t11453 * t4252;
    (t12446, t12448, t12451, t12455, t12458, t12460, t12463)
}
