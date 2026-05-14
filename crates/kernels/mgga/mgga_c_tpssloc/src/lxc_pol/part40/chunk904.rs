//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 904/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk904<F: Float>(t248: F, t3051: F, t4347: F, t1041: F, t3114: F, t4630: F, t3101: F, t4650: F, t1020: F, t10508: F, t1616: F, t122: F, t247: F) -> (F, F, F, F, F) {
    let t13950 = t248 * t3051 * t4347;
    let t13952 = t1041 * t13950 / 3456.0;
    let t13959 = t3114 * t4630 / 2304.0;
    let t13961 = t248 * t3101 * t4650;
    let t13963 = t1020 * t13961 / 2304.0;
    let t13965 = t248 * t10508 * t1616;
    let t13966 = t1020 * t13965;
    let t13969 = t247 * t122;
    (t13952, t13959, t13963, t13966, t13969)
}
