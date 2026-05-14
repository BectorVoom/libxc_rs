//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1313/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1313<F: Float>(t11678: F, t1227: F, t15507: F, t15654: F, t1653: F, t1734: F, t1737: F, t1748: F, t19033: F, t22275: F, t22301: F, t3578: F, t4582: F, t4972: F, t53087: F, t6211: F, t65444: F, t65464: F, t72161: F, t72181: F, t72183: F, t72389: F, t72398: F, t72967: F, t77606: F, t77621: F) -> (F,) {
    let t78689 = t15507 * t22275 / 48.0 - t72161 / 36.0 + t65444 / 216.0 - t1227 * t4582 * t4972 * t77621 / 576.0 + 5.0 / 384.0 * t1227 * t4582 * t15654 * t77606 + t72181 / 384.0 - 209.0 / 648.0 * t72389 * t1737 + 209.0 / 972.0 * t72398 * t1748 - 19.0 / 216.0 * t19033 * t6211 - t72183 / 576.0 - t53087 * t22301 / 144.0 + 19.0 / 144.0 * t72967 * t1737 - t11678 * t3578 * t65464 * t1653 * t1734 / 192.0;
    (t78689,)
}
