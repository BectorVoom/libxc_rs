//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1255/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1255<F: Float>(t42245: F, t47787: F, t59657: F, t68442: F, t76574: F, t76578: F, t76583: F, t76587: F, t76591: F, t76595: F, t76599: F, t59688: F, t59694: F, t68444: F, t68446: F, t68448: F, t68494: F, t68498: F, t76610: F, t76614: F, t76618: F, t76622: F, t76626: F) -> (F, F) {
    let t77427 = 0.71030123456790123454e-1 * t47787 - 0.50735802469135802467e-1 * t76574 - 0.17123333333333333333e-1 * t76578 - 0.3044148148148148148e-1 * t59657 + 0.2283111111111111111e0 * t76583 - 0.11415555555555555555e0 * t76587 - 0.41095999999999999999e0 * t76591 + 0.41095999999999999998e0 * t76595 - 0.34246666666666666665e-1 * t76599 + t42245 + 0.13698666666666666667e0 * t68442;
    let t77440 = 0.22831111111111111111e-1 * t68444 + 0.25367901234567901233e-1 * t68446 - 0.9132444444444444444e-1 * t68448 + 0.4566222222222222222e-1 * t68494 - 0.13698666666666666667e0 * t68498 - 0.4566222222222222222e-1 * t76610 + 0.41096e0 * t76614 - 0.61644e0 * t76618 + 0.10274e0 * t76622 + 0.13698666666666666667e0 * t76626 + 0.9132444444444444444e-1 * t59688 - 0.45662222222222222221e-1 * t59694;
    (t77427, t77440)
}
