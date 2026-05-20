//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1416;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta368<F: Float>(t1213: F, t15730: F, t11789: F, t1653: F, t248: F, t1227: F, t15437: F, t3505: F, t3576: F, t5064: F, t13969: F, t4988: F, t1725: F, t698: F, t1174: F, t225: F, t4941: F, t5053: F, t3701: F, t5356: F, t5168: F, t592: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15731, t15735, t15737, t15740, t15743) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1416::<F>(t1213, t15730, t11789, t1653, t248, t1227, t15437, t3505, t3576, t5064, t13969, t4988);
        let (t15745, t15754, t15797, t15820, t15868, t15877) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1417::<F>(t1227, t15743, t1725, t698, t1174, t225, t4941, t5053, t3701, t5356, t5168, t592);
    (t15731, t15735, t15737, t15740, t15745, t15754, t15797, t15820, t15868, t15877)
}
