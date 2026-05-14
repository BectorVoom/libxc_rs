//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 918/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk918<F: Float>(t11661: F, t4582: F, t3576: F, t3604: F, t3585: F, t820: F, t1216: F, t3243: F, t1090: F, t3494: F, t3578: F, t10401: F, t3575: F, t3610: F, t3509: F, t3252: F) -> (F, F, F, F, F, F, F, F) {
    let t11662 = t4582 * t11661;
    let t11665 = t3604 * t3576;
    let t11668 = t820 * t3585;
    let t11669 = t1216 * t3243;
    let t11670 = t11668 * t11669;
    let t11673 = t3494 * t1090;
    let t11674 = t3578 * t11673;
    let t11677 = t3575 * t10401;
    let t11678 = t3610 * t11677;
    let t11679 = t3509 * t1090;
    let t11680 = t3578 * t11679;
    let t11683 = t3252 * t1216;
    (t11662, t11665, t11670, t11674, t11677, t11678, t11680, t11683)
}
