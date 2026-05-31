//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2577/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2577<F: Float>(t14858: F, t6102: F, t1157: F, t1164: F, t22228: F, t1763: F, t4700: F, t64548: F, t71255: F, t71313: F, t71315: F, t71317: F, t71319: F, t72045: F, t72047: F, t72050: F) -> (F, F, F) {
    let t72052 = F::cast_from(0.17544670867903938621e1_f64) * t14858 * t6102;
    let t72058 = F::cast_from(0.14035736694323150897e2_f64) * t1164 * t22228 * t1157;
    let t72059 = -F::cast_from(3.0_f64) * t1763 * t4700 * t64548 + t71255 + t71313 + t71315 + t71317 + t71319 + t72045 - t72047 + t72050 - t72052 + t72058;
    (t72052, t72058, t72059)
}
