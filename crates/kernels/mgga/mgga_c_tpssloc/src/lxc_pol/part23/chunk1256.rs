//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1256/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1256<F: Float>(t41655: F, t47787: F, t59657: F, t68442: F, t76574: F, t76578: F, t76583: F, t76587: F, t76591: F, t76595: F, t76599: F, t59688: F, t59694: F, t68444: F, t68446: F, t68448: F, t68494: F, t68498: F, t76610: F, t76614: F, t76618: F, t76622: F, t76626: F) -> (F, F) {
    let t77454 = 0.73871604938271604937e-1 * t47787 - 0.52765432098765432099e-1 * t76574 - 0.17808333333333333333e-1 * t76578 - 0.31659259259259259258e-1 * t59657 + 0.23744444444444444444e0 * t76583 - 0.11872222222222222222e0 * t76587 - 0.42739999999999999999e0 * t76591 + 0.42739999999999999999e0 * t76595 - 0.35616666666666666666e-1 * t76599 + t41655 + 0.14246666666666666667e0 * t68442;
    let t77467 = 0.23744444444444444444e-1 * t68444 + 0.26382716049382716049e-1 * t68446 - 0.94977777777777777776e-1 * t68448 + 0.47488888888888888888e-1 * t68494 - 0.14246666666666666667e0 * t68498 - 0.47488888888888888888e-1 * t76610 + 0.4274e0 * t76614 - 0.6411e0 * t76618 + 0.10685e0 * t76622 + 0.14246666666666666667e0 * t76626 + 0.94977777777777777776e-1 * t59688 - 0.47488888888888888888e-1 * t59694;
    (t77454, t77467)
}
