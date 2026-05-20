//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1297/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1297<F: Float>(t20825: F, t46387: F, t67099: F, t46196: F, t5660: F, t193: F, t202: F, t2752: F, t39316: F, t39320: F, t39373: F, t39397: F, t39400: F, t39408: F, t39411: F, t40679: F, t40685: F, t40708: F) -> (F, F, F, F) {
    let t75854 = F::new(96.0) * t46387 * t20825;
    let t75855 = F::cast_from(0.23392894490538584828e1_f64) * t67099;
    let t75856 = F::cast_from(0.14035736694323150897e2_f64) * t46196;
    let t75857 = t5660 * t5660;
    let t75862 = -F::new(3.0) * t193 * t202 * t2752 * t75857 + t39316 + t39320 + t39373 - t39397 - t39400 + t39408 + t39411 - t40679 - t40685 + t40708 + t75854 - t75855 + t75856;
    (t75854, t75855, t75856, t75862)
}
