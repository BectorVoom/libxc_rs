//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2046/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2046<F: Float>(t116: F, t1314: F, t9534: F, t1307: F, t133: F, t6600: F, t59: F, t9223: F, t120: F, t212: F, t22815: F, t67: F) -> (F, F, F, F) {
    let t40369 = t9534 * t1314 * t116;
    let t40372 = t40369 * t133 * t6600 * t1307;
    let t40394 = t59 * t9223;
    let t40399 = t116 * t67 * t22815 * t120 * t212;
    (t40369, t40372, t40394, t40399)
}
