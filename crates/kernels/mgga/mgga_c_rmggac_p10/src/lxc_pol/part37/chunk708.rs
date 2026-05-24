//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 708/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk708<F: Float>(t1330: F, t1343: F, t3056: F, t641: F, t3046: F, t507: F, t7190: F, t3148: F, t7716: F, t16130: F, t511: F, t1971: F) -> (F, F, F, F) {
    let t69760 = t3056 * t1330 * t1343 * t641;
    let t69788 = t507 * t7190 * t3046;
    let t69806 = t7716 * t3148;
    let t69807 = t511 * t16130;
    let t69808 = t1971 * t69807;
    (t69760, t69788, t69806, t69808)
}
