//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 857/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk857<F: Float>(t15875: F, t15877: F, t15890: F, t15895: F, t19591: F, t11982: F, t11984: F, t1799: F, t193: F, t20077: F, t20354: F, t20355: F, t20356: F, t3918: F, t5160: F, t5161: F, t571: F, t6463: F, t9457: F, t9476: F, t9484: F) -> (F, F, F, F, F, F) {
    let t20360 = 24.0 * t15875;
    let t20361 = 24.0 * t15877;
    let t20365 = 0.51947577317044391276e2 * t15890;
    let t20366 = 0.17544670867903938621e1 * t15895;
    let t20370 = 12.0 * t19591;
    let t20371 = -9.0 * t1799 * t20077 * t3918 + 6.0 * t193 * t20356 * t571 - 3.0 * t5160 * t5161 * t6463 + t11982 - t11984 - t20354 + t20355 - t20360 - t20361 - t20365 - t20366 - t20370 - t9457 + t9476 + t9484;
    (t20360, t20361, t20365, t20366, t20370, t20371)
}
