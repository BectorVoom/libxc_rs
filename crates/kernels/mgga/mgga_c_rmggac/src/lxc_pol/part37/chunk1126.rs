//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1126/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1126<F: Float>(t14275: F, t14702: F, t14709: F, t14712: F, t14918: F, t14919: F, t15667: F, t15671: F, t15674: F, t15677: F, t15930: F, t15932: F, t15934: F, t15936: F, t73679: F, t73680: F) -> F {
    let t80553 = t15667 - t15930 + t15932 + t73679 + t15671 - t15674 - t15677 + t14702 + t14275 - t15934 + t14918 + t14919 + t73680 - t14709 - t14712 - t15936;
    t80553
}
