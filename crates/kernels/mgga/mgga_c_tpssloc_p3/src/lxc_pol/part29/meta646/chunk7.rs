//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2139/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2139<F: Float>(t25303: F, t6579: F, t1509: F, t6624: F, t13456: F, t1888: F, t6646: F, t13450: F, t23110: F, t23185: F, t4292: F, t25288: F, t81591: F) -> (F, F, F, F, F, F) {
    let t87565 = t6579 * t25303;
    let t87566 = F::cast_from(0.76763589786250567036e-1_f64) * t87565;
    let t87567 = t6624 * t1509;
    let t87575 = t1888 * t6646 * t13456;
    let t87578 = t1888 * t6646 * t13450;
    let t87581 = t23185 * t23110 * t4292;
    let t87582 = F::cast_from(0.82246703342411321824e-2_f64) * t87581;
    let t87583 = t81591 * t25288;
    (t87566, t87567, t87575, t87578, t87582, t87583)
}
