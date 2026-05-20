//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1678/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1678<F: Float>(t23168: F, t7480: F, t6547: F, t7489: F, t23237: F, t7488: F, t1880: F, t4300: F, t6571: F, t6553: F, t1519: F, t214: F) -> (F, F, F, F, F, F, F, F) {
    let t25209 = t23168 * t7480;
    let t25211 = t6547 * t7489;
    let t25213 = t23237 * t7488;
    let t25214 = t1880 * t25213;
    let t25216 = t6571 * t4300;
    let t25217 = t6553 * t25216;
    let t25218 = t1880 * t25217;
    let t25224 = t214 * t1519;
    (t25209, t25211, t25213, t25214, t25216, t25217, t25218, t25224)
}
