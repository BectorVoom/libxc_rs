//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2558/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2558<F: Float>(t14855: F, t3411: F, t14933: F, t300: F, t1166: F, t3401: F, t1155: F, t3395: F, t1695: F, t11292: F, t1164: F, t3404: F, t4857: F) -> (F, F, F, F, F) {
    let t51806 = F::cast_from(0.30762056574649219973e4_f64) * t3411 * t14855;
    let t51807 = t300 * t14933;
    let t51809 = F::cast_from(0.17544670867903938621e1_f64) * t51807 * t1166;
    let t51810 = t300 * t3401;
    let t51811 = t3395 * t1155;
    let t51814 = F::cast_from(0.10526802520742363173e2_f64) * t51810 * t1695 * t51811;
    let t51818 = F::cast_from(0.31168546390226634765e3_f64) * t1164 * t11292 * t4857 * t3404;
    (t51806, t51809, t51811, t51814, t51818)
}
