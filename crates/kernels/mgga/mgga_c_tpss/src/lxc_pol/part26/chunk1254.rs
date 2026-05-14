//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1254/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1254<F: Float>(t2376: F, t339: F, t5726: F, t1250: F, t31297: F, t522: F, t1712: F, t31814: F, t2436: F, t580: F, t5585: F, t8096: F, t1699: F, t8202: F, t5550: F, t790: F) -> (F, F, F, F, F, F, F, F) {
    let t60749 = t339 * t5726 * t2376;
    let t60750 = t60749 * t1250;
    let t60811 = t31297 * t522;
    let t60951 = t1712 * t31814;
    let t60960 = t2436 * t580;
    let t60996 = t5585 * t8096;
    let t61024 = t1699 * t8202;
    let t61025 = 595.0 / 5184.0 * t61024;
    let t61033 = t339 * t5550 * t790;
    (t60749, t60750, t60811, t60951, t60960, t60996, t61025, t61033)
}
