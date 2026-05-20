//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1134/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1134<F: Float>(t23168: F, t30664: F, t30643: F, t6547: F, t23109: F, t23110: F, t232: F, t59: F, t828: F, t23062: F, t30700: F, t240: F, t241: F, t2627: F, t812: F) -> (F, F, F, F, F) {
    let t112743 = t23168 * t30664;
    let t112760 = t6547 * t30643;
    let t112778 = t23109 * t23110 * t59 * t828 * t232;
    let t112784 = t23062 * t30700;
    let t112792 = t812 * t2627 * t240 * t241;
    (t112743, t112760, t112778, t112784, t112792)
}
