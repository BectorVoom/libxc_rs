//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 685/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk685<F: Float>(t3238: F, t3295: F, t3339: F, t3346: F, t4721: F, t4726: F, t4731: F, t4735: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F, t4773: F, t4776: F, t4779: F) -> (F,) {
    let t4819 = -0.17648625e1 * t4749 + 0.3529725e1 * t4757 + t3339 - 0.17215833333333333333e0 * t3238 - 0.17215833333333333333e0 * t4721 - 0.34431666666666666667e0 * t4726 + 0.103295e1 * t4731 + 0.516475e0 * t4735 + 0.31558125e0 * t4765 + 0.6311625e0 * t4767 + t3346 - 0.69463333333333333333e-1 * t3295 - 0.69463333333333333333e-1 * t4770 - 0.34731666666666666667e-1 * t4773 + 0.20839e0 * t4776 + 0.104195e0 * t4779;
    (t4819,)
}
