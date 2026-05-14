//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1067/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1067<F: Float>(t2331: F, t5488: F, t666: F, t5468: F, t9384: F, t659: F, t1444: F, t2: F, t584: F, t2341: F, t5396: F, t9212: F, t95: F, t5480: F, t9398: F, t662: F) -> (F, F, F, F, F, F, F) {
    let t19482 = t2331 * t5488;
    let t19483 = t19482 * t666;
    let t19488 = t9384 * t5468;
    let t19489 = t19488 * t659;
    let t19492 = t1444 * t2;
    let t19493 = t19492 * t584;
    let t19498 = t2341 * t5396;
    let t19499 = t19498 * t659;
    let t19503 = -t584 - 3.0 * t9212;
    let t19504 = t95 * t19503;
    let t19513 = t9398 * t5480;
    let t19514 = t19513 * t662;
    (t19483, t19489, t19493, t19499, t19503, t19504, t19514)
}
