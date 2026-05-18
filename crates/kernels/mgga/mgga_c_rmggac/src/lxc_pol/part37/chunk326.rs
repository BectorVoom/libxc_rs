//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 326/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk326<F: Float>(t141: F, t2: F, t7: F, t677: F, t202: F, t3112: F) -> (F, F, F, F, F, F) {
    let t3121 = F::new(1.0) / t141;
    let t3122 = t2 * t3121;
    let t3123 = t3122 * t7;
    let t3124 = t3123 * t677;
    let t3127 = t202 * t202;
    let t3128 = t3112 * t3127;
    (t3121, t3122, t3123, t3124, t3127, t3128)
}
