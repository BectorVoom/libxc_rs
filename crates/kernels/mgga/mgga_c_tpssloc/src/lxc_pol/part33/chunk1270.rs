//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1270/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1270<F: Float>(t28101: F, t80958: F, t1827: F, t91285: F, t19815: F, t6944: F, t22765: F, t6422: F, t22783: F, t6431: F, t1831: F, t91160: F) -> (F, F, F, F, F, F) {
    let t97238 = t80958 * t28101;
    let t97240 = t91285 * t1827;
    let t97246 = t19815 * t6944;
    let t97253 = t22765 * t6422;
    let t97261 = t22783 * t6431;
    let t97263 = t91160 * t1831;
    (t97238, t97240, t97246, t97253, t97261, t97263)
}
