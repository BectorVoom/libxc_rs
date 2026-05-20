//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1895/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1895<F: Float>(t12652: F, t605: F, t12661: F, t4017: F, t645: F, t72: F, t1433: F, t2241: F, t12568: F, t608: F, t2251: F, t3953: F) -> (F, F, F, F, F, F) {
    let t90153 = t605 * t12652;
    let t90160 = t605 * t12661;
    let t90177 = t72 * t4017 * t645;
    let t90196 = t72 * t1433 * t2241;
    let t90202 = t12568 * t608;
    let t90205 = t3953 * t2251;
    (t90153, t90160, t90177, t90196, t90202, t90205)
}
