//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 876/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk876<F: Float>(t3448: F, t4928: F, t11588: F, t1714: F, t3451: F, t3447: F, t14818: F, t14781: F, t14710: F, t1716: F, t698: F, t1174: F, t3435: F, t4889: F, t135: F, t4930: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15320 = t3448 * t4928;
    let t15338 = t11588 * t1714;
    let t15339 = t15338 * t3451;
    let t15341 = 0.18518518518518518518e-3 * t3447 * t15339;
    let t15347 = 2.0 / 27.0 * t14818;
    let t15348 = 4.0 / 9.0 * t14781;
    let t15349 = 2.0 / 9.0 * t14710;
    let t15363 = t698 * t1716;
    let t15364 = t1174 * t15363;
    let t15366 = t4889 * t3435;
    let t15372 = t135 * t4930;
    (t15320, t15338, t15341, t15347, t15348, t15349, t15364, t15366, t15372)
}
