//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta371<F: Float>(t1227: F, t15743: F, t1725: F, t698: F, t1174: F, t225: F, t4941: F, t5053: F, t3701: F, t5356: F, t5168: F, t592: F) -> (F, F, F, F, F, F, F) {
        let (t15745, t15753, t15754, t15797, t15820, t15868, t15877) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1424::<F>(t1227, t15743, t1725, t698, t1174, t225, t4941, t5053, t3701, t5356, t5168, t592);
    (t15745, t15753, t15754, t15797, t15820, t15868, t15877)
}
