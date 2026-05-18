//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1124/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1124<F: Float>(t15640: F, t15643: F, t15644: F, t15645: F, t15646: F, t15648: F, t15656: F, t15911: F, t15914: F, t15915: F, t15916: F, t15917: F, t15918: F, t15919: F, t15920: F, t15921: F) -> F {
    let t80550 = -t15911 + t15640 + t15643 + t15644 + t15645 - t15646 + t15914 + t15648 + t15915 - t15916 + t15917 + t15918 + t15919 - t15920 + t15921 + t15656;
    t80550
}
