//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 336/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk336<F: Float>(t1059: F, t1060: F, t1049: F, t383: F, t1003: F, t1058: F, t353: F, t384: F, t1055: F, t1050: F, t1052: F, t388: F, t991: F) -> (F, F, F, F, F) {
    let t1061 = t1059 * t1060;
    let t1063 = t383 * t1049;
    let t1065 = t1003 * t384 + t1058 * t1061 + t1063 * t353;
    let t1066 = t1055 * t1065;
    let t1068 = t1050 * t388 - t1052 * t1066 + t388 * t991;
    (t1061, t1063, t1065, t1066, t1068)
}
