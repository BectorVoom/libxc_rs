//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1058/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1058<F: Float>(t2632: F, t4233: F, t4180: F, t4181: F, t2639: F, t5619: F, t5614: F, t1484: F, t4119: F, t2701: F, t820: F, t5544: F, t776: F) -> (F, F, F, F, F, F) {
    let t16935 = t2632 * t4233;
    let t16937 = t4180 * t4181 * t16935;
    let t16940 = t2639 * t5619;
    let t16942 = t2639 * t5614;
    let t16944 = t1484 * t4119;
    let t16946 = t2701 * t820 * t16944;
    let t16949 = t5544 * t776;
    (t16935, t16937, t16940, t16942, t16946, t16949)
}
