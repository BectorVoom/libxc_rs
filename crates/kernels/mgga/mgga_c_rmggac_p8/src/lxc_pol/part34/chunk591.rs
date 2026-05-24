//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 591/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk591<F: Float>(t15163: F, t7204: F, t15166: F, t7192: F, t15169: F, t8620: F, t3069: F, t8659: F, t3077: F, t8365: F, t128: F, t589: F) -> (F, F, F, F, F, F) {
    let t15187 = t7204 * t15163;
    let t15189 = t7192 * t15166;
    let t15191 = t8620 * t15169;
    let t15197 = t8659 * t3069;
    let t15199 = t8365 * t3077;
    let t15203 = t128 * t589;
    (t15187, t15189, t15191, t15197, t15199, t15203)
}
