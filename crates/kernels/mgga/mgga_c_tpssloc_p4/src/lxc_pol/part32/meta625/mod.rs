//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2033;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta625<F: Float>(t86843: F, t4119: F, t857: F, t23168: F, t25342: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F, t25229: F, t794: F, t23164: F, t6555: F, t7480: F, t81632: F, t23030: F, t25035: F, t23228: F, t7479: F, t81573: F, t25059: F, t6562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86844, t86849, t86869, t86870, t86873, t86887, t86893) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2033::<F>(t86843, t4119, t857, t23168, t25342, t25345, t82038, t1519, t213, t225, t25229, t794);
        let (t86896, t86903, t86911, t86916, t86928) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2034::<F>(t23164, t6555, t86893, t7480, t81632, t23030, t25035, t23228, t7479, t81573, t25059, t6562, t794);
    (t86844, t86849, t86869, t86870, t86873, t86887, t86893, t86896, t86903, t86911, t86916, t86928)
}
