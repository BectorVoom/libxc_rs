//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2089/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2089<F: Float>(t16752: F, t232: F, t860: F, t2732: F, t5612: F, t1509: F, t1519: F) -> (F, F, F, F) {
    let t16753 = t16752 * t232;
    let t16754 = t860 * t16753;
    let t16756 = t2732 * t5612;
    let t16758 = t1519 * t1509;
    (t16753, t16754, t16756, t16758)
}
