//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1318/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1318<F: Float>(t78298: F, t78302: F, t78304: F, t78310: F, t78312: F, t78314: F, t78318: F, t78320: F, t78327: F, t78329: F, t78331: F, t78333: F, t50834: F, t71335: F, t71337: F, t77959: F, t77963: F, t77967: F, t77971: F, t77975: F, t77979: F, t77983: F, t77989: F, t77992: F, t77995: F, t77998: F) -> (F, F) {
    let t78794 = -t78298 + t78302 - t78304 + t78310 - t78312 - t78314 - t78318 - t78320 + t78327 + t78329 + t78331 + t78333;
    let t78809 = 0.55570666666666666666e0 * t77959 - 0.10805407407407407407e0 * t77963 - 0.104195e0 * t77967 + 0.62517e0 * t77971 - 0.125034e1 * t77975 + 0.250068e1 * t77979 + 0.104195e0 * t77983 + 0.27785333333333333333e0 * t71335 - 0.166712e1 * t71337 - 0.21424148148148148148e1 * t50834 + 0.123954e2 * t77989 + 0.516475e0 * t77992 - 0.15302962962962962963e1 * t77995 + 0.309885e1 * t77998;
    (t78794, t78809)
}
