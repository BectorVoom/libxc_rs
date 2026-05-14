//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 447/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk447<F: Float>(t112: F, t1441: F, t1408: F, t95: F, t50: F, tau1: F) -> (F, F, F, F, F) {
    let t1442 = t1441 * t112;
    let t1444 = t1408 / 2.0;
    let t1445 = t95 * t1444;
    let t1447 = tau1 * t50;
    let t1449 = -t1444;
    (t1442, t1444, t1445, t1447, t1449)
}
