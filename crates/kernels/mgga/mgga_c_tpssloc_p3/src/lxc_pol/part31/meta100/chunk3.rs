//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 614/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk614<F: Float>(t123: F, t131: F, t2387: F, t2390: F, t693: F, t119: F, t63: F, t133: F) -> (F, F, F, F) {
    let t2396 = F::new(1.0)/F::sqrt(t123);
    let t2397 = t2396 * t131;
    let t2398 = t2397 * t2387;
    let t2400 = t693 * t2390;
    let t2402 = t119 * t63;
    let t2403 = t133 * t2402;
    (t2397, t2398, t2400, t2403)
}
