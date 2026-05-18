//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1290/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1290<F: Float>(t18464: F, t4484: F, t1646: F, t60749: F, t60750: F, t19506: F, t5570: F, t18495: F, t6259: F, t20509: F, t2436: F, t6353: F, t8096: F) -> (F, F, F, F, F, F, F) {
    let t65643 = t18464 * t4484;
    let t65647 = t60749 * t1646;
    let t65650 = F::new(119.0) / F::new(864.0) * t60750;
    let t65667 = t19506 * t5570;
    let t65871 = t6259 * t18495;
    let t66281 = t20509 * t2436;
    let t66299 = t6353 * t8096;
    (t65643, t65647, t65650, t65667, t65871, t66281, t66299)
}
