//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1108/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1108<F: Float>(t40: F, t10121: F, t870: F, t2517: F, t2519: F, t195: F, t632: F, t2244: F, t2250: F, t2433: F, t39097: F, t39103: F, t39110: F, t73: F, t9258: F, t9427: F, t9430: F, zeta_threshold: F) -> (F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t40622 = t10121 * t870;
    let t40626 = t2519 * t2517;
    let t40627 = 6.0 * t40626;
    let t40632 = 1.0 / t195 / t632;
    let t40645 = piecewise3(t146, 0.0, 40.0 / 81.0 * t40632 * t39097 - 16.0 / 9.0 * t9427 * t2244 * t2250 + 4.0 / 3.0 * t2433 * t39103 + 16.0 / 9.0 * t9430 * t9258 + 4.0 / 3.0 * t73 * t39110);
    (t40622, t40627, t40645)
}
