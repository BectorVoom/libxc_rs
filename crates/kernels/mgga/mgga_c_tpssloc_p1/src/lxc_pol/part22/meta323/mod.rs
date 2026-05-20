//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1508;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1509;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta323<F: Float>(t5168: F, t592: F, t5166: F, t588: F, t5187: F, t571: F, t11981: F, t2528: F, t5154: F, t172: F, t5151: F, t763: F, t2535: F, t12461: F, t1845: F, t118: F, t1787: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15877, t15880, t15883, t15889, t15890, t15892, t15894) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1508::<F>(t5168, t592, t5166, t588, t5187, t571, t11981, t2528, t5154, t172, t5151, t763);
        let (t15895, t15898, t15899, t15908) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1509::<F>(t2535, t5154, t5166, t592, t12461, t1845, t118, t1787);
    (t15877, t15880, t15883, t15889, t15890, t15892, t15894, t15895, t15898, t15899, t15908)
}
