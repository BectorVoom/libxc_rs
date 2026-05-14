//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 465/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk465<F: Float>(t1693: F, t1695: F, t510: F, t220: F, t523: F, t64: F, t529: F, t1705: F, t532: F) -> (F, F, F, F) {
    let t1762 = t1693 * t510 * t1695;
    let t1765 = t220 * t523 * t64;
    let t1766 = t1765 * t529;
    let t1771 = t1705 * t532;
    (t1762, t1765, t1766, t1771)
}
