//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 826/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk826<F: Float>(t522: F, t9212: F, t9214: F, t3824: F, t592: F, t1287: F, t2221: F, t9216: F, t9218: F, t1294: F, t9713: F, t25: F, t526: F, t28: F, t528: F, t9722: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12044 = 24.0 * t9212 * t522;
    let t12045 = t9214 * t522;
    let t12046 = 144.0 * t12045;
    let t12048 = 12.0 * t592 * t3824;
    let t12052 = t2221 * t1287;
    let t12053 = 36.0 * t12052;
    let t12054 = t9216 * t522;
    let t12055 = 240.0 * t12054;
    let t12057 = 120.0 * t9218 * t522;
    let t12059 = 0.5848223622634646207e0 * t1294 * t9713;
    let t12061 = 1.0 / t526 / t25;
    let t12072 = 1.0 / t528 / t28;
    let t12087 = 0.10389515463408878255e3 * t1294 * t9722;
    (t12044, t12046, t12048, t12053, t12055, t12057, t12059, t12061, t12072, t12087)
}
