//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2576/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2576<F: Float>(t300: F, t71322: F, t71664: F, t71712: F, t71752: F, t71791: F, t71828: F, t71868: F, t72041: F, t18926: F, t4869: F, t1164: F, t14960: F, t6085: F) -> (F, F, F) {
    let t72045 = t300 * (t71322 + t71664 + t71712 + t71752 + t71791 + t71828 + t71868 + t72041);
    let t72047 = F::cast_from(0.17544670867903938621e1_f64) * t4869 * t18926;
    let t72050 = F::cast_from(0.35089341735807877242e1_f64) * t1164 * t14960 * t6085;
    (t72045, t72047, t72050)
}
