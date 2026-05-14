//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1051/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1051<F: Float>(t27948: F, t67: F, t1864: F, t7441: F, t7445: F, t5441: F, t71: F, t1863: F, t5389: F, t79: F, t72: F, t1410: F, t3953: F, t1433: F, t1437: F, t5445: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27949 = t27948 * t67;
    let t27950 = t27949 * t1864;
    let t27953 = t7441 * t7445;
    let t27956 = t71 * t5441;
    let t27957 = t1863 * t27956;
    let t27960 = t79 * t5389;
    let t27961 = t72 * t27960;
    let t27966 = t3953 * t1410;
    let t27971 = t1433 * t1437;
    let t27972 = t72 * t27971;
    let t27975 = t79 * t5445;
    (t27949, t27950, t27953, t27956, t27957, t27961, t27966, t27972, t27975)
}
