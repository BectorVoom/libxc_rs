//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 263/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk263<F: Float>(t1206: F, t1243: F, t1215: F, t491: F, t357: F, t475: F, t1235: F, t493: F, t1201: F, t470: F, t494: F, t1241: F, t1191: F, t1236: F, t1238: F, t498: F) -> (F, F, F, F, F, F, F) {
    let t1244 = t1206 * t1243;
    let t1245 = t491 * t1215;
    let t1246 = t357 * t475;
    let t1247 = t1245 * t1246;
    let t1249 = t493 * t1235;
    let t1251 = t1201 * t494 + t1244 * t1247 + t1249 * t470;
    let t1252 = t1241 * t1251;
    let t1254 = t1191 * t498 + t1236 * t498 - t1238 * t1252;
    (t1244, t1246, t1247, t1249, t1251, t1252, t1254)
}
