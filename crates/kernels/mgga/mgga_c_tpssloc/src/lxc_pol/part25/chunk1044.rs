//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1044/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1044<F: Float>(t33: F, t39046: F, t608: F, t9239: F, t1864: F, t2241: F, t9231: F, t645: F, t6509: F, t22530: F, t72: F, t2307: F, t641: F, t9228: F, t2303: F, t2240: F, t2251: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t83710 = t39046 * t33;
    let t83717 = t9239 * t608;
    let t83718 = t1864 * t2241;
    let t83722 = t9231 * t608;
    let t83728 = t6509 * t645;
    let t83734 = t72 * t22530 * t645;
    let t83737 = t1864 * t2307;
    let t83745 = t72 * t641 * t2241;
    let t83748 = t9228 * t608;
    let t83771 = t72 * t2303 * t645;
    let t83778 = t2240 * t2251;
    (t83710, t83717, t83718, t83722, t83728, t83734, t83737, t83745, t83748, t83771, t83778)
}
