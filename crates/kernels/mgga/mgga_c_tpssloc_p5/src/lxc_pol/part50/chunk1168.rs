//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1168/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1168<F: Float>(t1888: F, t23270: F, t25170: F, t112678: F, t112676: F, t118476: F, t118479: F, t118481: F, t118484: F, t118488: F, t118491: F, t118498: F, t118499: F, t118500: F, t13463: F, t25168: F, t25188: F, t40889: F, t4272: F, t6632: F, t8352: F, t8353: F) -> F {
    let t118503 = F::cast_from(0.9869604401089358619e-1_f64) * t1888 * t23270 * t25170;
    let t118506 = F::cast_from(0.82246703342411321825e-2_f64) * t112678;
    let t118509 = F::cast_from(24.0_f64) * t25168 * t40889 * t4272 * t8352 + F::cast_from(2.0_f64) * t13463 * t8353 + F::cast_from(4.0_f64) * t25188 * t6632 - t112676 + t118476 + t118479 - t118481 + t118484 - t118488 + t118491 + t118498 + t118499 + t118500 - t118503 + t118506;
    t118509
}
