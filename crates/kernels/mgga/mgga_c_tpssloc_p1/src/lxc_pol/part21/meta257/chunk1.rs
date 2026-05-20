//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1496/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1496<F: Float>(t202: F, t243: F, t2229: F, t61: F, t119: F, t212: F, t343: F, t984: F, t3034: F, t334: F) -> (F, F, F, F, F) {
    let t6589 = F::new(1.0) / t243 / t202;
    let t6597 = F::new(1.0) / t61 / t2229;
    let t6600 = t119 * t212;
    let t6733 = t984 * t343;
    let t6739 = F::new(1.0) / t3034 / t334;
    (t6589, t6597, t6600, t6733, t6739)
}
