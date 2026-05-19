//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 963/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk963<F: Float>(t3403: F, t4857: F, t15026: F, t3623: F, t1706: F, t3428: F, t135: F, t457: F, t4936: F, t1174: F, t3431: F, t4912: F) -> (F, F, F, F, F) {
    let t15218 = t4857 * t3403;
    let t15245 = t15026 * t3623;
    let t15265 = t1706 * t3428;
    let t15281 = t135 * t457;
    let t15282 = t15281 * t4936;
    let t15284 = F::cast_from(0.55555555555555555554e-3_f64) * t1174 * t15282;
    let t15285 = t3431 * t4912;
    (t15218, t15245, t15265, t15284, t15285)
}
