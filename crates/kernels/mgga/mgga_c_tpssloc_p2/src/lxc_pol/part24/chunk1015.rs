//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1015/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1015<F: Float>(t3247: F, t405: F, t974: F, t11147: F, t461: F, t9288: F, t457: F, t63: F, t221: F, t456: F, t1186: F, t698: F) -> (F, F, F, F, F) {
    let t11545 = F::new(1.0) / t405 / t3247;
    let t11546 = t974 * t11545;
    let t11547 = t461 * t11147;
    let t11548 = t11547 * t9288;
    let t11549 = t11546 * t11548;
    let t11552 = t63 * t457;
    let t11553 = t11552 * t461;
    let t11554 = t221 * t11553;
    let t11556 = F::cast_from(0.3086419753086419753e-3_f64) * t456 * t11554;
    let t11557 = t698 * t1186;
    (t11545, t11549, t11552, t11556, t11557)
}
