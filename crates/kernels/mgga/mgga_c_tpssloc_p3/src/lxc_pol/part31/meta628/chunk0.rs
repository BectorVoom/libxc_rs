//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1885/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1885<F: Float>(t22765: F, t6422: F, t19921: F, t6952: F, t19926: F, t22756: F, t22783: F, t6431: F, t1831: F, t91160: F, t19815: F, t6951: F) -> (F, F, F, F, F, F, F) {
    let t97253 = t22765 * t6422;
    let t97255 = t6952 * t19921;
    let t97257 = t6952 * t19926;
    let t97259 = t22756 * t6422;
    let t97261 = t22783 * t6431;
    let t97263 = t91160 * t1831;
    let t97265 = t19815 * t6951;
    (t97253, t97255, t97257, t97259, t97261, t97263, t97265)
}
