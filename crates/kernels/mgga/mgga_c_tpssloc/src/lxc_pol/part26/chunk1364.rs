//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1364/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1364<F: Float>(t11713: F, t11715: F, t11717: F, t24649: F, t24658: F, t2131: F, t82985: F, t7325: F, t10469: F, t1209: F, t478: F, t11720: F, t3032: F, sigma2: F) -> (F, F, F, F, F) {
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    let t86149 = t24658 * t24649;
    let t86154 = t2131 * t82985;
    let t86155 = t86154 * t7325;
    let t86157 = t10469 * t1209 * t478;
    let t86158 = t11720 * t3032;
    (t86146, t86149, t86155, t86157, t86158)
}
