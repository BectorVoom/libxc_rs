//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta357<F: Float>(t5166: F, t588: F, t11981: F, t2528: F, t5154: F, t172: F, t5151: F, t763: F, t2535: F, t592: F, t118: F, t1787: F) -> (F, F, F, F, F, F, F) {
        let (t15880, t15889, t15890, t15894, t15895, t15898, t15908) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1274::<F>(t5166, t588, t11981, t2528, t5154, t172, t5151, t763, t2535, t592, t118, t1787);
    (t15880, t15889, t15890, t15894, t15895, t15898, t15908)
}
