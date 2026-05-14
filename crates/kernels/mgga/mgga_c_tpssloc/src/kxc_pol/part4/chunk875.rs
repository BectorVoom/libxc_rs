//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 875/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk875<F: Float>(t1706: F, t3428: F, t135: F, t457: F, t4936: F, t1174: F, t3431: F, t4912: F, t11583: F, t3961: F, t11529: F, t1709: F, t3432: F, t4889: F, t3450: F, t3966: F) -> (F, F, F, F, F, F, F) {
    let t15265 = t1706 * t3428;
    let t15281 = t135 * t457;
    let t15282 = t15281 * t4936;
    let t15284 = 0.55555555555555555554e-3 * t1174 * t15282;
    let t15285 = t3431 * t4912;
    let t15287 = 0.18518518518518518518e-3 * t1174 * t15285;
    let t15293 = t11583 * t3961;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15307 = t4889 * t3432;
    let t15313 = t3450 * t3966;
    (t15265, t15284, t15287, t15293, t15300, t15307, t15313)
}
