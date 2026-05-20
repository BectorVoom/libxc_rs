//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2175/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2175<F: Float>(t80722: F, t80744: F, t81264: F, t90605: F, t90609: F, t90646: F, t93438: F, t93445: F, t97509: F, t97513: F, t97516: F, t1992: F, t22635: F, t26354: F, t5353: F) -> (F, F) {
    let t97519 = -t90605 - F::cast_from(0.49348022005446793095e-1_f64) * t90609 + F::cast_from(0.63969658155208805863e-1_f64) * t80722 - t80744 + F::cast_from(0.82246703342411321825e-2_f64) * t97509 - F::cast_from(0.82246703342411321825e-2_f64) * t97513 + F::cast_from(0.3289868133696452873e-1_f64) * t97516 + t93438 + t90646 + F::cast_from(0.26044789391763585244e-1_f64) * t81264 - t93445;
    let t97524 = t1992 * t22635 * t26354 * t5353;
    (t97519, t97524)
}
