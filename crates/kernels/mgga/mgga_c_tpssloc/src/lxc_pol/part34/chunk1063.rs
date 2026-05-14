//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1063/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1063<F: Float>(t107063: F, t107065: F, t107067: F, t107070: F, t107074: F, t107077: F, t107084: F, t107086: F, t107088: F, t107090: F, t91149: F, t91167: F, t97219: F, t97238: F, t97240: F, t97253: F, t97261: F, t97263: F, t97283: F) -> (F,) {
    let t107802 = t107063 / 64.0 + t107065 / 128.0 + t107067 / 64.0 - 7.0 / 48.0 * t97219 + t107070 / 64.0 - 0.10173934535723378495e0 * t97238 + 7.0 / 192.0 * t97240 - t107074 / 256.0 + 7.0 / 384.0 * t97253 + 5.0 / 64.0 * t107077 + 7.0 / 96.0 * t97261 + 7.0 / 48.0 * t97263 - 119.0 / 288.0 * t91149 - 35.0 / 96.0 * t97283 - 0.67826230238155856633e-1 * t91167 - t107084 / 768.0 - t107086 / 256.0 - t107088 / 256.0 - t107090 / 128.0;
    (t107802,)
}
