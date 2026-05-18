//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 594/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk594<F: Float>(t552: F, t793: F, t1986: F, t3141: F, t2060: F, t8975: F, t1550: F, t8946: F, t903: F, t7577: F, t8936: F, t739: F) -> (F, F, F, F, F, F, F, F) {
    let t15234 = t793 * t552;
    let t15235 = t1986 * t15234;
    let t15236 = t3141 * t15235;
    let t15238 = t2060 * t8975;
    let t15239 = t1550 * t15238;
    let t15240 = F::new(0.5987120850931904282e-1) * t15239;
    let t15241 = t2060 * t8946;
    let t15242 = t903 * t15241;
    let t15243 = F::new(0.8980681276397856423e-1) * t15242;
    let t15244 = t7577 * t8936;
    let t15245 = t739 * t15244;
    (t15235, t15236, t15238, t15240, t15241, t15243, t15244, t15245)
}
