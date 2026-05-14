//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1158/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1158<F: Float>(t24848: F, t86036: F, t24594: F, t24847: F, t974: F, t27551: F, t7327: F, t135: F, t7284: F, t11713: F, t11715: F, t11717: F, t2131: F, t82985: F, t7325: F, t10469: F, t1209: F, t478: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t86037 = t86036 * t24848;
    let t86076 = t24847 * t974 * t24594;
    let t86077 = t7327 * t27551;
    let t86094 = t24847 * t135 * t7284;
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    let t86154 = t2131 * t82985;
    let t86155 = t86154 * t7325;
    let t86157 = t10469 * t1209 * t478;
    (t86037, t86076, t86077, t86094, t86146, t86155, t86157)
}
