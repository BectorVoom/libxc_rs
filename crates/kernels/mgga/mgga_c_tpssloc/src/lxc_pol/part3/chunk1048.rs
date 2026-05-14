//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1048/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1048<F: Float>(t15268: F, t4934: F, t1714: F, t3469: F, t460: F, t1178: F, t12606: F, t1177: F, t135: F, t457: F, t4936: F, t1174: F, t3431: F, t4912: F, t1090: F, t7319: F) -> (F, F, F, F, F, F) {
    let t15269 = t4934 * t15268;
    let t15273 = t1714 * t3469 * t460;
    let t15274 = t4934 * t15273;
    let t15277 = t1178 * t12606;
    let t15278 = t1177 * t15277;
    let t15281 = t135 * t457;
    let t15282 = t15281 * t4936;
    let t15284 = 0.55555555555555555554e-3 * t1174 * t15282;
    let t15285 = t3431 * t4912;
    let t15287 = 0.18518518518518518518e-3 * t1174 * t15285;
    let t15288 = t7319 * t1090;
    (t15269, t15274, t15278, t15284, t15287, t15288)
}
