//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1393/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1393<F: Float>(t20201: F, t72: F, t79: F, t1433: F, t5445: F, t20288: F, t5398: F, t20218: F, t605: F, t1410: F, t19299: F, t28025: F, t4028: F) -> (F, F, F, F, F, F, F) {
    let t106836 = t72 * t79 * t20201;
    let t106842 = t72 * t1433 * t5445;
    let t106849 = t72 * t79 * t20288;
    let t106853 = t72 * t79 * t5398;
    let t106855 = t605 * t20218;
    let t106862 = t19299 * t1410;
    let t106889 = F::cast_from(6.0_f64) * t4028 * t28025;
    (t106836, t106842, t106849, t106853, t106855, t106862, t106889)
}
