//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1146/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1146<F: Float>(t1041: F, t21138: F, t248: F, t3051: F, t21134: F, t14508: F, t17667: F, t17611: F, t4641: F, t10480: F, t21391: F, t3101: F, t10457: F, t21118: F, t1020: F, t21595: F) -> (F, F, F, F, F, F, F) {
    let t70166 = t1041 * t248 * t3051 * t21138;
    let t70199 = t1041 * t248 * t3051 * t21134;
    let t70209 = t14508 * t17667;
    let t70214 = t4641 * t17611;
    let t70227 = t10480 * t248 * t3101 * t21391;
    let t70239 = t1041 * t248 * t10457 * t21118;
    let t70346 = t1020 * t248 * t3101 * t21595;
    (t70166, t70199, t70209, t70214, t70227, t70239, t70346)
}
