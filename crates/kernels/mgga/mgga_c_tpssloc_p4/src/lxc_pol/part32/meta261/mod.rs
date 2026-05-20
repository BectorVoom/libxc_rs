//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1174;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1175;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta261<F: Float>(t533: F, t6995: F, t1390: F, t1983: F, t1388: F, t3701: F, t2019: F, t1873: F, t3938: F, t671: F, t3941: F, t1401: F, t6534: F, t2108: F, t33: F, t2240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6996, t6997, t6998, t6999, t7000, t7001, t7014, t7015, t7017, t7019) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1174::<F>(t533, t6995, t1390, t1983, t1388, t3701, t2019, t1873, t3938, t671, t3941, t1401, t6534);
        let t7245 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1175::<F>(t2108, t33);
        let t7246 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1176::<F>(t2240, t7245);
    (t6996, t6997, t6998, t6999, t7000, t7001, t7014, t7015, t7017, t7019, t7245, t7246)
}
