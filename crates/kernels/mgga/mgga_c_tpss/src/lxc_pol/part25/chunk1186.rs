//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1186/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1186<F: Float>(t18450: F, t4462: F, t60731: F, t4473: F, t60738: F, t18464: F, t4484: F, t1646: F, t60749: F, t60750: F, t19506: F, t5570: F, t18495: F, t6259: F, t20509: F, t2436: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65628 = t18450 * t4462;
    let t65634 = 35.0 / 108.0 * t60731;
    let t65639 = t60738 * t4473;
    let t65643 = t18464 * t4484;
    let t65647 = t60749 * t1646;
    let t65650 = 119.0 / 864.0 * t60750;
    let t65667 = t19506 * t5570;
    let t65871 = t6259 * t18495;
    let t66281 = t20509 * t2436;
    (t65628, t65634, t65639, t65643, t65647, t65650, t65667, t65871, t66281)
}
