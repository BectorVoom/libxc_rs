//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1476/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1476<F: Float>(t11677: F, t11907: F, t11769: F, t13969: F, t3515: F, t11904: F, t11702: F, t3536: F, t11709: F, t11745: F, t11651: F, t11734: F) -> (F, F, F, F, F, F) {
    let t45134 = t11907 * t11677;
    let t45148 = t3515 * t13969 * t11769;
    let t45162 = t11904 * t11677;
    let t45167 = t3536 * t11702;
    let t45169 = t11709 * t11745;
    let t45171 = t11734 * t11651;
    (t45134, t45148, t45162, t45167, t45169, t45171)
}
