//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2535/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2535<F: Float>(t1036: F, t13751: F, t10422: F, t14229: F, t3070: F, t14234: F, t42488: F, t1022: F, t4649: F, t41666: F, t43398: F, t14036: F) -> (F, F, F, F, F, F) {
    let t48446 = t13751 * t1036;
    let t48460 = t3070 * t10422 * t14229;
    let t48463 = t3070 * t42488 * t14234;
    let t48477 = t4649 * t1022;
    let t48496 = t43398 * t41666;
    let t48548 = t3070 * t42488 * t14036;
    (t48446, t48460, t48463, t48477, t48496, t48548)
}
