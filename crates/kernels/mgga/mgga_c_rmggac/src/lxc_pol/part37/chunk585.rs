//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 585/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk585<F: Float>(t15098: F, t3851: F, t13902: F, t556: F, t13905: F, t2842: F, t3046: F, t551: F) -> (F, F, F, F) {
    let t15099 = t3851 * t15098;
    let t15101 = t13902 * t556;
    let t15103 = t13905 * t2842;
    let t15105 = t3046 * t551;
    (t15099, t15101, t15103, t15105)
}
