//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1418/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1418<F: Float>(t33: F, t39046: F, t608: F, t9239: F, t1864: F, t2241: F, t1863: F, t9231: F, t22550: F, t6505: F, t645: F, t6509: F) -> (F, F, F, F, F, F) {
    let t83710 = t39046 * t33;
    let t83717 = t9239 * t608;
    let t83718 = t1864 * t2241;
    let t83719 = t1863 * t83718;
    let t83722 = t9231 * t608;
    let t83725 = t6505 * t22550;
    let t83728 = t6509 * t645;
    (t83710, t83717, t83719, t83722, t83725, t83728)
}
