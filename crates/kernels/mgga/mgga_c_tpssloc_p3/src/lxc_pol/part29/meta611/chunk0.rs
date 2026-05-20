//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2050/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2050<F: Float>(t1404: F, t7415: F, t2174: F, t3931: F, t24954: F, t580: F, t111: F, t112: F, t24542: F, t2109: F, t83718: F, t22550: F, t7255: F) -> (F, F, F, F, F, F, F, F) {
    let t85405 = t7415 * t1404;
    let t85407 = t3931 * t2174;
    let t85412 = t24954 * t580;
    let t85416 = t7415 * t111;
    let t85423 = t24954 * t112;
    let t85428 = t24542 * t111;
    let t85463 = t2109 * t83718;
    let t85470 = t7255 * t22550;
    (t85405, t85407, t85412, t85416, t85423, t85428, t85463, t85470)
}
