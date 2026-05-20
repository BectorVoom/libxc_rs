//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1819/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1819<F: Float>(t1864: F, t2241: F, t608: F, t9231: F, t645: F, t6509: F, t2307: F, t2240: F, t2251: F, t22573: F, t6875: F, t24486: F, t576: F) -> (F, F, F, F, F, F, F) {
    let t83718 = t1864 * t2241;
    let t83722 = t9231 * t608;
    let t83728 = t6509 * t645;
    let t83737 = t1864 * t2307;
    let t83778 = t2240 * t2251;
    let t83886 = t6875 * t22573;
    let t84031 = t576 * t24486;
    (t83718, t83722, t83728, t83737, t83778, t83886, t84031)
}
