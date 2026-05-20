//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2504/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2504<F: Float>(t41115: F, t4191: F, t41107: F, t4166: F, t9670: F, t831: F, t12890: F, t751: F, t12932: F, t2427: F, t13133: F, t2430: F) -> (F, F, F, F, F, F, F) {
    let t47079 = t41115 * t4191;
    let t47081 = t41107 * t4191;
    let t47092 = t4166 * t9670;
    let t47093 = t47092 * t831;
    let t47160 = t12890 * t751;
    let t47163 = t2427 * t12932;
    let t47165 = t13133 * t2430;
    (t47079, t47081, t47092, t47093, t47160, t47163, t47165)
}
