//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1217/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1217<F: Float>(t24736: F, t3523: F, t11813: F, t7338: F, t3566: F, t7344: F, t11801: F, t7345: F, t11708: F, t24728: F, t11713: F, t11715: F, t11717: F, t24649: F, t24658: F, t2131: F, t82985: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t86124 = t24736 * t3523;
    let t86126 = t11813 * t7338;
    let t86129 = t3566 * t7344;
    let t86136 = t7345 * t11801;
    let t86140 = t11708 * t24728;
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    let t86149 = t24658 * t24649;
    let t86154 = t2131 * t82985;
    (t86124, t86126, t86129, t86136, t86140, t86146, t86149, t86154)
}
