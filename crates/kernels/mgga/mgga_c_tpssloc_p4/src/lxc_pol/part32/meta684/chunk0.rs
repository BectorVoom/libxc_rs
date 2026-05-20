//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2125/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2125<F: Float>(t1864: F, t5445: F, t2240: F, t5399: F, t3953: F, t3961: F, t3967: F, t1437: F, t4017: F, t72: F, t1433: F, t4021: F) -> (F, F, F, F, F, F) {
    let t96469 = t1864 * t5445;
    let t96473 = t2240 * t5399;
    let t96479 = t3953 * t3961;
    let t96482 = t3953 * t3967;
    let t96502 = t72 * t4017 * t1437;
    let t96506 = t72 * t1433 * t4021;
    (t96469, t96473, t96479, t96482, t96502, t96506)
}
