//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1248/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1248<F: Float>(t59657: F, t60168: F, t60173: F, t60204: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F, t77042: F, t77073: F, t77076: F, t41959: F, t59688: F, t59694: F, t76574: F, t76578: F, t76583: F, t76591: F, t76599: F, t76614: F, t76622: F, t76893: F, t76896: F, t76909: F, t76915: F) -> (F, F) {
    let t77189 = 0.22076e0 * t68502 + 0.132456e1 * t68504 - 0.44152e0 * t68506 + 0.247573125e0 * t77042 + 0.11038e1 * t60168 - 0.5519e0 * t60173 - 0.53675555555555555556e0 * t59657 + 0.99342e0 * t76880 + 0.16504875e0 * t77073 - 0.485484375e1 * t77076 - 0.18396666666666666667e0 * t60204 - 0.82785e-1 * t76877 - 0.8585111111111111111e-1 * t76887 - 0.82785e-1 * t76890;
    let t77204 = -0.99342e0 * t76893 + 0.44152e0 * t76896 + 0.198684e1 * t76909 + 0.49671e0 * t76915 - 0.89459259259259259259e0 * t76574 - 0.301925e0 * t76578 + 0.40256666666666666666e1 * t76583 - 0.72462e1 * t76591 - 0.60384999999999999999e0 * t76599 + 0.72462e1 * t76614 + 0.181155e1 * t76622 + 0.16102666666666666667e1 * t59688 - 0.80513333333333333336e0 * t59694 + t41959;
    (t77189, t77204)
}
