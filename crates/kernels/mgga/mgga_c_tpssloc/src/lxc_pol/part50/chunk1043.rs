//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1043/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1043<F: Float>(t31220: F, t532: F, t2029: F, t7002: F, t2022: F, t7020: F, t1395: F, t8509: F, t31288: F, t576: F, t112: F, t31253: F, t111: F, t8496: F, t33164: F, t580: F) -> (F, F, F, F, F, F, F, F) {
    let t114418 = t532 * t31220;
    let t114439 = t7002 * t2029;
    let t114441 = t2022 * t7020;
    let t114449 = t1395 * t8509;
    let t114451 = t576 * t31288;
    let t114475 = t31253 * t112;
    let t114495 = t8496 * t111;
    let t118373 = t33164 * t580;
    (t114418, t114439, t114441, t114449, t114451, t114475, t114495, t118373)
}
