//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1251/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1251<F: Float>(t59657: F, t60168: F, t60173: F, t60204: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F, t77042: F, t77073: F, t77076: F, t42212: F, t59688: F, t59694: F, t76574: F, t76578: F, t76583: F, t76591: F, t76599: F, t76614: F, t76622: F, t76893: F, t76896: F, t76909: F, t76915: F) -> (F, F) {
    let t77272 = 0.27785333333333333333e0 * t68502 + 0.166712e1 * t68504 - 0.55570666666666666668e0 * t68506 + 0.94674375e0 * t77042 + 0.13892666666666666667e1 * t60168 - 0.69463333333333333334e0 * t60173 - 0.91817777777777777776e0 * t59657 + 0.125034e1 * t76880 + 0.6311625e0 * t77073 - 0.6618234375e1 * t77076 - 0.23154444444444444445e0 * t60204 - 0.104195e0 * t76877 - 0.10805407407407407407e0 * t76887 - 0.104195e0 * t76890;
    let t77287 = -0.125034e1 * t76893 + 0.55570666666666666666e0 * t76896 + 0.250068e1 * t76909 + 0.62517e0 * t76915 - 0.15302962962962962963e1 * t76574 - 0.516475e0 * t76578 + 0.68863333333333333334e1 * t76583 - 0.123954e2 * t76591 - 0.103295e1 * t76599 + 0.123954e2 * t76614 + 0.309885e1 * t76622 + 0.27545333333333333333e1 * t59688 - 0.13772666666666666666e1 * t59694 + t42212;
    (t77272, t77287)
}
