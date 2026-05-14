//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1079/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1079<F: Float>(t12333: F, t12351: F, t450: F, t1112: F, t242: F, t3090: F, t4056: F, t1125: F, t1128: F, t11846: F, t1501: F, t9666: F, t3062: F, t4258: F, t1114: F, t581: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12352 = t12333 + t12351;
    let t12353 = t12352 * t450;
    let t12355 = t242 * t1112 * t12353;
    let t12359 = t242 * t3090 * t4056;
    let t12361 = t1125 * t12359 / 3456.0;
    let t12363 = t242 * t1128 * t11846;
    let t12367 = t242 * t9666 * t1501;
    let t12368 = t1125 * t12367;
    let t12371 = t4258 * t3062 / 432.0;
    let t12372 = t1114 * t581;
    (t12352, t12355, t12359, t12361, t12363, t12367, t12368, t12371, t12372)
}
