//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 921/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk921<F: Float>(t14406: F, t14407: F, t14408: F, t14409: F, t14410: F, t14411: F, t14423: F, t14426: F, t15452: F, t15453: F, t15454: F, t15455: F, t70661: F, t70667: F, t70668: F) -> F {
    let t76589 = t70661 + t14406 + t14407 + t15452 - t14408 + t14409 - t14410 - t14411 - t15453 + t15454 + t15455 - t70667 + t70668 + t14423 - t14426;
    t76589
}
