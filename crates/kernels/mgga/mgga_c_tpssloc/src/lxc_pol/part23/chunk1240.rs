//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1240/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1240<F: Float>(t5705: F, t2815: F, t41904: F, t47787: F, t59657: F, t68442: F, t76574: F, t76578: F, t76583: F, t76587: F, t76591: F, t76595: F, t76599: F, t59688: F, t59694: F, t68444: F, t68446: F, t68448: F, t68494: F, t68498: F, t76610: F, t76614: F, t76618: F, t76622: F, t76626: F) -> (F, F, F, F) {
    let t77041 = t5705 * t5705;
    let t77042 = t2815 * t77041;
    let t77058 = 112.0 / 81.0 * t47787 - 80.0 / 81.0 * t76574 - t76578 / 3.0 - 16.0 / 27.0 * t59657 + 40.0 / 9.0 * t76583 - 20.0 / 9.0 * t76587 - 8.0 * t76591 + 8.0 * t76595 - 2.0 / 3.0 * t76599 + t41904 + 8.0 / 3.0 * t68442;
    let t77071 = 4.0 / 9.0 * t68444 + 40.0 / 81.0 * t68446 - 16.0 / 9.0 * t68448 + 8.0 / 9.0 * t68494 - 8.0 / 3.0 * t68498 - 8.0 / 9.0 * t76610 + 8.0 * t76614 - 12.0 * t76618 + 2.0 * t76622 + 8.0 / 3.0 * t76626 + 16.0 / 9.0 * t59688 - 8.0 / 9.0 * t59694;
    (t77041, t77042, t77058, t77071)
}
