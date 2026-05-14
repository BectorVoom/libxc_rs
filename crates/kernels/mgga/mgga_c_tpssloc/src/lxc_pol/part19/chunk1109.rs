//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1109/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1109<F: Float>(t52: F, t197: F, t636: F, t2244: F, t2250: F, t2440: F, t39097: F, t39103: F, t39110: F, t76: F, t9258: F, t9438: F, t9441: F, t40645: F, t145: F, t185: F, t2531: F, t9892: F, zeta_threshold: F) -> (F, F, F) {
    let t150 = t52 <= zeta_threshold;
    let t40647 = 1.0 / t197 / t636;
    let t40660 = piecewise3(t150, 0.0, 40.0 / 81.0 * t40647 * t39097 + 16.0 / 9.0 * t9438 * t2244 * t2250 + 4.0 / 3.0 * t2440 * t39103 + 16.0 / 9.0 * t9441 * t9258 - 4.0 / 3.0 * t76 * t39110);
    let t40661 = t40645 + t40660;
    let t40663 = t145 * t40661 * t185;
    let t40667 = t2531 * t9892;
    (t40661, t40663, t40667)
}
