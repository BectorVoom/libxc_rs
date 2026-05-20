//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2066/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2066<F: Float>(t23197: F, t6547: F, t23257: F, t6562: F, t794: F, t23012: F, t6568: F, t225: F, t23211: F, t23205: F, t82038: F, t23242: F, t81979: F) -> (F, F, F, F, F, F) {
    let t82230 = t6547 * t23197;
    let t82236 = t6562 * t794 * t23257;
    let t82259 = t23012 * t6568;
    let t82287 = t23211 * t225;
    let t82294 = t82038 * t23205;
    let t82296 = t81979 * t23242;
    (t82230, t82236, t82259, t82287, t82294, t82296)
}
