//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2156/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2156<F: Float>(t19815: F, t6944: F, t1354: F, t1827: F, t91278: F, t26233: F, t5289: F, t22765: F, t6422: F, t19921: F, t6952: F, t19926: F) -> (F, F, F, F, F, F) {
    let t97246 = t19815 * t6944;
    let t97247 = t97246 * t1354;
    let t97249 = t91278 * t1827;
    let t97251 = t26233 * t5289;
    let t97253 = t22765 * t6422;
    let t97255 = t6952 * t19921;
    let t97257 = t6952 * t19926;
    (t97247, t97249, t97251, t97253, t97255, t97257)
}
