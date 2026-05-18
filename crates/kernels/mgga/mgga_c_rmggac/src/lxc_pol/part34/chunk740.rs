//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 740/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk740<F: Float>(t1986: F, t2213: F, t118: F, t495: F, t699: F, t69583: F, t14413: F, t638: F, t7292: F, t14417: F, t2046: F, t7297: F) -> (F, F, F, F, F) {
    let t71346 = t1986 * t2213;
    let t71366 = t1986 * t118 * t699 * t495;
    let t71369 = F::new(0.17347588262831798124e-3) * t69583;
    let t71372 = t638 * t7292 * t14413;
    let t71373 = F::new(0.81300399444200075504e-3) * t71372;
    let t71375 = t2046 * t7297 * t14417;
    (t71346, t71366, t71369, t71373, t71375)
}
