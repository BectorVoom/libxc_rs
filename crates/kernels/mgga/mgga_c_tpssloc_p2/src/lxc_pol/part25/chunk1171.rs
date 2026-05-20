//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1171/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1171<F: Float>(t23012: F, t6568: F, t23270: F, t25038: F, t2553: F, t258: F, t776: F, t6553: F, t6554: F, t81984: F, t9458: F, t23205: F, t82038: F) -> (F, F, F, F) {
    let t82259 = t23012 * t6568;
    let t82266 = t25038 * t23270 * t258 * t2553 * t776;
    let t82282 = t81984 * t6553 * t6554 * t9458;
    let t82294 = t82038 * t23205;
    (t82259, t82266, t82282, t82294)
}
