//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1227/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1227<F: Float>(t81520: F, t82333: F, t1054: F, t2775: F, t1065: F, t2244: F, t23547: F, t381: F, t23310: F, t23384: F, t23460: F, t6686: F) -> (F, F, F, F, F, F) {
    let t82334 = t81520 + t82333;
    let t82342 = t1054 * t2775;
    let t82343 = t2244 * t1065;
    let t82357 = t23547 * t381;
    let t82380 = t23384 * t23310;
    let t82382 = t23460 * t6686;
    (t82334, t82342, t82343, t82357, t82380, t82382)
}
