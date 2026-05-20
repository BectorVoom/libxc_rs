//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1170/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1170<F: Float>(t23197: F, t6547: F, t23222: F, t23237: F, t6552: F, t23257: F, t6562: F, t794: F, t10109: F, t225: F, t10111: F, t1880: F, t6553: F) -> (F, F, F, F) {
    let t82230 = t6547 * t23197;
    let t82233 = t6552 * t23237 * t23222;
    let t82236 = t6562 * t794 * t23257;
    let t82252 = t225 * t10109;
    let t82255 = t1880 * t6553 * t82252 * t10111;
    (t82230, t82233, t82236, t82255)
}
