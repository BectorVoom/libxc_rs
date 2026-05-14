//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 188/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk188<F: Float>(t1314: F, t253: F, t40: F, t41: F, t21: F, t22: F) -> (F, F, F, F, F) {
    let t1315 = t253 * t1314;
    let t1318 = t40 * t40;
    let t1320 = 1.0 / t41 / t1318;
    let t1321 = t21 * t1320;
    let t1322 = t22 * t22;
    (t1315, t1318, t1320, t1321, t1322)
}
