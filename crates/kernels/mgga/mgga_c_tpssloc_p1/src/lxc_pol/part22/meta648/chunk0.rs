//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2188/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2188<F: Float>(t19573: F, t588: F, t592: F, t118: F, t2375: F, t6320: F, t12300: F, t6422: F, t12365: F, t1358: F, t19836: F, t12250: F, t6387: F) -> (F, F, F, F, F, F, F) {
    let t57227 = t588 * t19573;
    let t57229 = t592 * t19573;
    let t57235 = t6320 * t118 * t2375;
    let t57308 = t12300 * t6422;
    let t57310 = t12365 * t6422;
    let t57324 = t19836 * t1358;
    let t57342 = t6387 * t12250;
    (t57227, t57229, t57235, t57308, t57310, t57324, t57342)
}
