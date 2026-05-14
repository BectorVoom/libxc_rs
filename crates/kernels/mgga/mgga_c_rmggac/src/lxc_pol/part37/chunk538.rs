//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 538/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk538<F: Float>(t15238: F, t1550: F, t2060: F, t8946: F, t903: F, t7577: F, t8936: F, t739: F, t618: F, t664: F) -> (F, F, F, F, F, F) {
    let t15239 = t1550 * t15238;
    let t15240 = 0.5987120850931904282e-1 * t15239;
    let t15241 = t2060 * t8946;
    let t15242 = t903 * t15241;
    let t15243 = 0.8980681276397856423e-1 * t15242;
    let t15244 = t7577 * t8936;
    let t15245 = t739 * t15244;
    let t15246 = 0.5987120850931904282e-1 * t15245;
    let t15252 = t664 * t618;
    (t15240, t15241, t15243, t15244, t15246, t15252)
}
