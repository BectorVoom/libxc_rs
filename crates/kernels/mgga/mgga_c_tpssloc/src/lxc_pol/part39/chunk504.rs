//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 504/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk504<F: Float>(t1799: F, t210: F, t214: F, t1313: F, t1315: F, t1322: F, t562: F, t119: F, t225: F) -> (F, F, F, F, F, F) {
    let t1804 = t210 * t214 * t1799;
    let t1807 = -t1313 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t1804 - t1322;
    let t1808 = t1807 * t562;
    let t1810 = t119 * t1799;
    let t1811 = t210 * t1810;
    let t1814 = t1807 * t225;
    (t1804, t1807, t1808, t1810, t1811, t1814)
}
