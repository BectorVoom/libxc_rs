//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1366/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1366<F: Float>(t1284: F, t6279: F, t1673: F, t5760: F, t20131: F, t546: F, t1781: F, t4562: F, t20082: F, t550: F, t1275: F, t6296: F, t1276: F, t13253: F, t1789: F, t3: F, t60628: F, t62104: F, t62167: F, t62169: F, t66064: F) -> (F,) {
    let t66161 = 2.0 * t6279 * t1284;
    let t66163 = 2.0 * t5760 * t1673;
    let t66165 = 2.0 * t546 * t20131;
    let t66167 = 2.0 * t1781 * t4562;
    let t66173 = 2.0 * t20082 * t550;
    let t66175 = 2.0 * t1275 * t6296;
    let t66176 = t3 * t550 * t66064 + 2.0 * t1276 * t20131 + t13253 * t1789 + t60628 + t62104 + 2.0 * t62167 + t62169 + t66161 + t66163 + t66165 + t66167 + t66173 + t66175;
    (t66176,)
}
