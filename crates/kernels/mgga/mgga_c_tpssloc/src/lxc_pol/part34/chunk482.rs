//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 482/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk482<F: Float>(t1709: F, t3431: F, t1174: F, t3439: F, t60: F, t461: F, t1409: F, t3450: F, t3448: F, t135: F, t1716: F, t1714: F, t457: F, t974: F, t1721: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4896 = t3431 * t1709;
    let t4897 = t1174 * t4896;
    let t4899 = t60 * t3439;
    let t4900 = t4899 * t461;
    let t4904 = t3450 * t1409;
    let t4908 = t3448 * t461;
    let t4916 = t135 * t1716;
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    let t4934 = t974 * t457;
    let t4945 = t1721 * t225;
    (t4896, t4897, t4899, t4900, t4904, t4908, t4916, t4917, t4919, t4934, t4945)
}
