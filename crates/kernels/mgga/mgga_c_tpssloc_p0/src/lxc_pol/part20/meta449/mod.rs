//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta449<F: Float>(t3378: F, t4882: F, t1164: F, t3411: F, t4879: F, t11433: F, t3396: F, t4874: F, t11424: F, t4745: F, t11185: F, t4786: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15036, t15038, t15040, t15041, t15043, t15044, t15046, t15048, t15050) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1899::<F>(t3378, t4882, t1164, t3411, t4879, t11433, t3396, t4874, t11424, t4745, t11185, t4786);
    (t15036, t15038, t15040, t15041, t15043, t15044, t15046, t15048, t15050)
}
