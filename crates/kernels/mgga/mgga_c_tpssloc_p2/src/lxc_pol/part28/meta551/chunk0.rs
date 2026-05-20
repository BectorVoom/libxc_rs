//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1821/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1821<F: Float>(t39054: F, t7025: F, t23966: F, t9231: F, t6492: F, t22527: F, t23967: F, t22531: F, t22519: F, t7032: F, t22537: F, t23998: F, t6495: F) -> (F, F, F, F, F, F, F, F) {
    let t84190 = t39054 * t7025;
    let t84195 = t9231 * t23966;
    let t84196 = t84195 * t6492;
    let t84198 = t23967 * t22527;
    let t84200 = t23967 * t22531;
    let t84203 = t22519 * t7032;
    let t84205 = t22537 * t7032;
    let t84207 = t6495 * t23998;
    (t84190, t84195, t84196, t84198, t84200, t84203, t84205, t84207)
}
