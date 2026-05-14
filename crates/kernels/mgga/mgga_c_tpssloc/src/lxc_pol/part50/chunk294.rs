//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 294/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk294<F: Float>(t496: F, t68: F, t1011: F, t1209: F, t1206: F, t1215: F, t491: F, t357: F, t475: F) -> (F, F, F, F, F, F) {
    let t1239 = t496 * t496;
    let t1240 = 1.0 / t1239;
    let t1241 = t68 * t1240;
    let t1243 = t1011 * t1209;
    let t1244 = t1206 * t1243;
    let t1245 = t491 * t1215;
    let t1246 = t357 * t475;
    (t1239, t1241, t1243, t1244, t1245, t1246)
}
