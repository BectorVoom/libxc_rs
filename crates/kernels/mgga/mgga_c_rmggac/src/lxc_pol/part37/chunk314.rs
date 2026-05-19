//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 314/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk314<F: Float>(t21: F, t40: F, t1318: F, t2045: F) -> (F, F, F, F, F) {
    let t3051 = t21 * t21;
    let t3052 = F::sqrt(t40);
    let t3054 = F::new(1.0) / t3052 / t1318;
    let t3055 = t3051 * t3054;
    let t3056 = t3055 * t2045;
    (t3051, t3052, t3054, t3055, t3056)
}
