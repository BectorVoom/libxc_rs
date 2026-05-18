//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 625/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk625<F: Float>(t14519: F, t14521: F, t14522: F, t15558: F, t15559: F, t15560: F, t15561: F, t15562: F, t15564: F, t15566: F, t15568: F, t15603: F) -> F {
    let t15605 = t15558 + t14519 + t15559 - t15560 - t15561 - t14521 + t14522 + t15562 + t15564 - t15566 - t15568 + t15603;
    t15605
}
