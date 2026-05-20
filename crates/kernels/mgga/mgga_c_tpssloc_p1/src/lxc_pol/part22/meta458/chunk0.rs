//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1829/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1829<F: Float>(t4049: F, t5396: F, t20215: F, t95: F, t5415: F, t1449: F, t5480: F, t9398: F, t4059: F, t5484: F, t103: F, t100: F, t104: F, t1447: F, t1450: F, t20312: F, t5475: F, t5481: F, t5485: F, t92: F, tau1: F) -> (F, F, F, F, F, F, F) {
    let t20315 = t4049 * t5396;
    let t20318 = F::new(3.0) * t20215;
    let t20319 = t95 * t20318;
    let t20322 = tau1 * t5415;
    let t20331 = t5480 * t1449;
    let t20332 = t9398 * t20331;
    let t20335 = t4059 * t5484;
    let t20338 = -t20318;
    let t20339 = t103 * t20338;
    let t20342 = -F::new(10.0) / F::new(27.0) * t92 * t20312 + F::new(10.0) / F::new(3.0) * t92 * t20315 + F::new(5.0) / F::new(3.0) * t92 * t20319 - F::new(440.0) / F::new(27.0) * t20322 * t104 + F::new(200.0) / F::new(9.0) * t5475 * t1450 - F::new(50.0) / F::new(9.0) * t1447 * t5481 - F::new(25.0) / F::new(3.0) * t1447 * t5485 - F::new(10.0) / F::new(27.0) * t100 * t20332 + F::new(10.0) / F::new(3.0) * t100 * t20335 + F::new(5.0) / F::new(3.0) * t100 * t20339;
    (t20315, t20318, t20319, t20322, t20331, t20338, t20342)
}
