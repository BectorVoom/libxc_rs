//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1365/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1365<F: Float>(t11713: F, t11717: F, t24727: F, t11708: F, t24732: F, t7337: F, t11651: F, t24733: F, t11797: F, t7345: F, t11724: F, t11731: F, t11741: F, t11781: F, t24664: F, t24670: F, t24706: F, t3518: F, t475: F, t7316: F, t86146: F, t86149: F, t86155: F, t86157: F, t86158: F) -> F {
    let t86164 = t11713 * t24727 * t11717;
    let t86167 = t11708 * t24732;
    let t86171 = t11713 * t7337 * t11717;
    let t86174 = t24733 * t11651;
    let t86176 = t7345 * t11797;
    let t86182 = t86146 * t11724 / F::new(256.0) - F::new(0.60559134141210586284e-3) * t86149 * t24664 + F::new(0.30279567070605293142e-3) * t86149 * t24670 + F::new(0.10093189023535097714e-3) * t86155 * t86157 * t86158 * t475 - t86164 * t11731 / F::new(256.0) - t86167 * t3518 / F::new(512.0) + t86171 * t11741 / F::new(1536.0) - t86174 / F::new(768.0) - t86176 / F::new(1152.0) - F::new(5.0) / F::new(2592.0) * t7345 * t11781 + F::new(0.30279567070605293142e-3) * t7316 * t24706;
    t86182
}
