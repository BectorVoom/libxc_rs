//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1219/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1219<F: Float>(t11724: F, t11731: F, t11741: F, t11781: F, t24664: F, t24670: F, t24706: F, t3518: F, t475: F, t7316: F, t7345: F, t86146: F, t86149: F, t86155: F, t86157: F, t86158: F, t86164: F, t86167: F, t86171: F, t86174: F, t86176: F) -> (F,) {
    let t86182 = t86146 * t11724 / 256.0 - 0.60559134141210586284e-3 * t86149 * t24664 + 0.30279567070605293142e-3 * t86149 * t24670 + 0.10093189023535097714e-3 * t86155 * t86157 * t86158 * t475 - t86164 * t11731 / 256.0 - t86167 * t3518 / 512.0 + t86171 * t11741 / 1536.0 - t86174 / 768.0 - t86176 / 1152.0 - 5.0 / 2592.0 * t7345 * t11781 + 0.30279567070605293142e-3 * t7316 * t24706;
    (t86182,)
}
