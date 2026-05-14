//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 872/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk872<F: Float>(t10620: F, t300: F, t2897: F, t961: F, t2940: F, t2948: F, t2928: F, t941: F) -> (F, F, F, F, F) {
    let t10622 = 0.19751673498613801407e-1 * t300 * t10620;
    let t10623 = t300 * t2897;
    let t10625 = 0.17544670867903938621e1 * t10623 * t961;
    let t10627 = 0.17544670867903938621e1 * t2940 * t2948;
    let t10629 = 1.0 / t2928 / t941;
    (t10622, t10623, t10625, t10627, t10629)
}
