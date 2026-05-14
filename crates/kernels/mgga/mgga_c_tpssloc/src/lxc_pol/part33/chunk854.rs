//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 854/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk854<F: Float>(t4059: F, t5484: F, t20318: F, t103: F, t100: F, t104: F, t1447: F, t1450: F, t20312: F, t20315: F, t20319: F, t20322: F, t20332: F, t5475: F, t5481: F, t5485: F, t92: F) -> (F,) {
    let t20335 = t4059 * t5484;
    let t20338 = -t20318;
    let t20339 = t103 * t20338;
    let t20342 = -10.0 / 27.0 * t92 * t20312 + 10.0 / 3.0 * t92 * t20315 + 5.0 / 3.0 * t92 * t20319 - 440.0 / 27.0 * t20322 * t104 + 200.0 / 9.0 * t5475 * t1450 - 50.0 / 9.0 * t1447 * t5481 - 25.0 / 3.0 * t1447 * t5485 - 10.0 / 27.0 * t100 * t20332 + 10.0 / 3.0 * t100 * t20335 + 5.0 / 3.0 * t100 * t20339;
    (t20342,)
}
