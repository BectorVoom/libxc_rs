//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 846/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk846<F: Float>(t20553: F, t550: F, t1343: F, t820: F, t1799: F, t6347: F, t3870: F, t20489: F, t20416: F, t210: F, t214: F, t20356: F) -> (F, F, F, F, F, F, F, F) {
    let t20554 = t20553 * t550;
    let t20556 = t1343 * t820 * t20554;
    let t20563 = t1799 * t6347;
    let t20565 = t3870 * t820 * t20563;
    let t20568 = t20489 * t550;
    let t20570 = t1343 * t820 * t20568;
    let t20576 = t210 * t214 * t20416;
    let t20582 = t210 * t214 * t20356;
    (t20554, t20556, t20563, t20565, t20568, t20570, t20576, t20582)
}
