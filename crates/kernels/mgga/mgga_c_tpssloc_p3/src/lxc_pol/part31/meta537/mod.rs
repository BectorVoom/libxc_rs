//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1752;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta537<F: Float>(t131: F, t22791: F, t9537: F, t1338: F, t225: F, t236: F, t1336: F, t2690: F, t6950: F, t1369: F, t22782: F, t3777: F, t15: F, t2229: F, t1361: F, t192: F, t1995: F, t22690: F, t2230: F, t22843: F, t213: F, t22842: F, t531: F, t598: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t80853, t80854, t80855, t80866, t80867, t80869) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1752::<F>(t131, t22791, t9537, t1338, t225, t236, t1336, t2690, t6950, t1369, t22782, t3777);
        let (t80881, t80885, t80887, t80888, t80893) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1753::<F>(t15, t2229, t1361, t192, t1995, t22690, t2230, t22843, t213, t22842, t531, t598);
    (t80853, t80854, t80855, t80866, t80867, t80869, t80881, t80885, t80887, t80888, t80893)
}
