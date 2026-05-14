//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1152/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1152<F: Float>(t12935: F, t193: F, t202: F, t2522: F, t2553: F, t39585: F, t39590: F, t39593: F, t40848: F, t40887: F, t41252: F, t41254: F, t41256: F, t41258: F, t41260: F, t41262: F, t41266: F, t41580: F, t766: F, t870: F, t9470: F) -> (F,) {
    let t41591 = -t39585 + t39590 + 3.0 * t193 * t766 * t40848 + t193 * t202 * (t40887 + t41580) * t870 + t41252 - t39593 - 18.0 * t2522 * t9470 * t2553 + 36.0 * t193 * t12935 * t2553 + t41254 - t41256 - t41258 - t41260 - t41262 - t41266;
    (t41591,)
}
