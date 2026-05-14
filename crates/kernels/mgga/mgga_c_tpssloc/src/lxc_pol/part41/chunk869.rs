//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 869/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk869<F: Float>(t3215: F, t390: F, t268: F, t405: F, t6546: F, t1091: F, t2394: F) -> (F, F, F, F) {
    let t11094 = 1.0 / t3215 / t390;
    let t11135 = t268 * t6546 * t405;
    let t11136 = 0.28842592592592592592e-1 * t11135;
    let t11137 = t2394 * t1091;
    (t11094, t11135, t11136, t11137)
}
