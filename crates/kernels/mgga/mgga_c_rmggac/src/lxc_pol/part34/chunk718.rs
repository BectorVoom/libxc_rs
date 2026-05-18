//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 718/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk718<F: Float>(t14362: F, t2190: F, t3144: F, t25561: F, t29: F, t3117: F, t3132: F, t3136: F, t13839: F, t2044: F, t352: F, t7554: F) -> (F, F, F, F) {
    let t70176 = t2190 * t14362 * t3144;
    let t70186 = t3117 * t25561 * t29;
    let t70188 = t3132 * t70186 * t3136;
    let t70194 = t13839 * t2044 * t7554 * t352;
    (t70176, t70186, t70188, t70194)
}
