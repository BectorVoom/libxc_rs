//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2531/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2531<F: Float>(t13823: F, t2960: F, t13816: F, t2970: F, t973: F, t13828: F, t10224: F, t4522: F, t13895: F, t1599: F, t2402: F, t13908: F) -> (F, F, F, F, F, F, F) {
    let t48297 = t2960 * t13823;
    let t48302 = t973 * t2970 * t13816;
    let t48317 = t2960 * t13828;
    let t48320 = t973 * t10224 * t4522;
    let t48328 = t2960 * t13895;
    let t48336 = t973 * t2402 * t1599;
    let t48338 = t2960 * t13908;
    (t48297, t48302, t48317, t48320, t48328, t48336, t48338)
}
