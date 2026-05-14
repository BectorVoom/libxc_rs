//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 886/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk886<F: Float>(t3700: F, t570: F, t111: F, t1395: F, t584: F, t9212: F, t9214: F, t9216: F, t9218: F, t9220: F, t3951: F, t604: F, t1406: F, t2239: F, t4025: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12461 = 1.0 / t3700 / t570;
    let t12524 = t1395 * t111;
    let t12560 = 0.348e1 * t584;
    let t12561 = 0.156e1 * t9212;
    let t12562 = 0.312e1 * t9214;
    let t12563 = 0.2312e3 * t9216;
    let t12564 = 0.3468e3 * t9218;
    let t12565 = 0.56952e3 * t9220;
    let t12568 = t3951 * t604;
    let t12571 = t1406 * t2239;
    let t12603 = 2.0 * t584;
    let t12604 = 6.0 * t9212;
    let t12725 = t4025 * t111;
    (t12461, t12524, t12560, t12561, t12562, t12563, t12564, t12565, t12568, t12571, t12603, t12604, t12725)
}
