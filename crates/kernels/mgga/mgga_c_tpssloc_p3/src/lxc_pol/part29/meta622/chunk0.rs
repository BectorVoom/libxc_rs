//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2064/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2064<F: Float>(t11801: F, t7345: F, t11708: F, t24728: F, t11713: F, t11715: F, t11717: F, t2131: F, t82985: F, t24727: F, t24732: F, t7337: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t86136 = t7345 * t11801;
    let t86140 = t11708 * t24728;
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    let t86154 = t2131 * t82985;
    let t86164 = t11713 * t24727 * t11717;
    let t86167 = t11708 * t24732;
    let t86171 = t11713 * t7337 * t11717;
    (t86136, t86140, t86146, t86154, t86164, t86167, t86171)
}
