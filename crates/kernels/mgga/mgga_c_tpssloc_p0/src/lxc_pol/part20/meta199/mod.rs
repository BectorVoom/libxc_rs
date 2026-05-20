//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta199<F: Float>(t1166: F, t4869: F, t1703: F, t3411: F, t1694: F, t3375: F, t1157: F, t1164: F, t1147: F, t1156: F, t4857: F, t3400: F) -> (F, F, F, F, F, F, F, F) {
        let (t4871, t4873, t4874, t4875, t4877, t4879, t4881, t4882) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1201::<F>(t1166, t4869, t1703, t3411, t1694, t3375, t1157, t1164, t1147, t1156, t4857, t3400);
    (t4871, t4873, t4874, t4875, t4877, t4879, t4881, t4882)
}
