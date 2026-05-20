//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1353/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1353<F: Float>(t10109: F, t225: F, t10111: F, t1880: F, t6553: F, t23012: F, t6568: F, t23270: F, t25038: F, t2553: F, t258: F, t776: F) -> (F, F, F) {
    let t82252 = t225 * t10109;
    let t82255 = t1880 * t6553 * t82252 * t10111;
    let t82259 = t23012 * t6568;
    let t82266 = t25038 * t23270 * t258 * t2553 * t776;
    (t82255, t82259, t82266)
}
