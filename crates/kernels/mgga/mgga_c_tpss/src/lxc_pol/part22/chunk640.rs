//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 640/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk640<F: Float>(t1114: F, t3060: F, t242: F, t1111: F, t453: F, t458: F, t1141: F, t2738: F) -> (F, F, F, F) {
    let t3061 = t3060 * t1114;
    let t3062 = t242 * t3061;
    let t3063 = t1111 * t3062;
    let t3065 = t453 * t458;
    let t3067 = t1141 * t3065 * t2738;
    (t3062, t3063, t3065, t3067)
}
