//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1036/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1036<F: Float>(t14651: F, t14653: F, t14979: F, t14983: F, t14984: F, t15510: F, t15511: F, t15512: F, t15513: F, t15514: F, t15518: F, t15522: F, t15900: F, t15902: F, t15903: F, t73666: F) -> F {
    let t79959 = t14651 + t73666 + t15510 - t15511 + t15512 + t15513 - t15514 + t14653 + t15900 + t15902 - t15903 + t15518 + t14979 + t14983 - t14984 - t15522;
    t79959
}
