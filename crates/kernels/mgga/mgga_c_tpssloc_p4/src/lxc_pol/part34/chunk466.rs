//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 466/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk466<F: Float>(t457: F, t697: F, t461: F, t221: F, t456: F, t1176: F, t135: F, t1089: F, t405: F) -> (F, F, F, F, F) {
    let t3426 = t697 * t457;
    let t3427 = t3426 * t461;
    let t3428 = t221 * t3427;
    let t3430 = F::cast_from(0.18518518518518518518e-3_f64) * t456 * t3428;
    let t3431 = t135 * t1176;
    let t3439 = F::new(1.0) / t405 / t1089;
    (t3426, t3428, t3430, t3431, t3439)
}
