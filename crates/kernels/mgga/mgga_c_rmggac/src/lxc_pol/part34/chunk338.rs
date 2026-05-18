//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 338/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk338<F: Float>(t118: F, t3204: F, t3074: F, t3078: F, t3184: F, t3185: F, t3187: F, t3190: F, t3193: F, t3196: F) -> F {
    let t3205 = t118 * t3204;
    let t3207 = -t3184 + t3185 - t3074 - F::new(0.31062809106223861415e-2) * t3078 + t3187 - t3190 + t3193 - t3196 + F::new(0.19957069503106347607e-1) * t3205;
    t3207
}
