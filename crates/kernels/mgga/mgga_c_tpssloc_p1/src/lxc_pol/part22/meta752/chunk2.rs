//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2527/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2527<F: Float>(t423: F, t71162: F, t71214: F, t1157: F, t1164: F, t21938: F, t3375: F, t1254: F, t19270: F, t4700: F, t5091: F, t71095: F, t71097: F, t71101: F, t71106: F, t71109: F, t71112: F, t71114: F, t71118: F) -> (F, F, F) {
    let t71217 = F::cast_from(0.621814e-1_f64) * (t71162 + t71214) * t423;
    let t71221 = F::cast_from(0.11696447245269292414e1_f64) * t1164 * t3375 * t21938 * t1157;
    let t71222 = -t1254 * t4700 * t71101 + F::cast_from(6.0_f64) * t19270 * t4700 * t5091 + t71095 - t71097 + t71106 - t71109 - t71112 + t71114 + t71118 - t71217 + t71221;
    (t71217, t71221, t71222)
}
