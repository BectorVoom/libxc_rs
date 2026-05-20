//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1834;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta562<F: Float>(t1408: F, t2745: F, t25365: F, t81547: F, t1530: F, t2553: F, t22960: F, t12971: F, t25: F, t2379: F, t4255: F, t606: F, t870: F, t25213: F, t6547: F, t22986: F, t23270: F, t25053: F, t4119: F, t857: F, t865: F, t4300: F, t776: F, t1888: F, t2717: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86806, t86810, t86815, t86816, t86821, t86825, t86830) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1834::<F>(t1408, t2745, t25365, t81547, t1530, t2553, t22960, t12971, t25, t2379, t4255, t606, t870);
        let (t86843, t86847, t86852, t86857, t86862) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1835::<F>(t25213, t6547, t22986, t23270, t25053, t2553, t4119, t857, t865, t4300, t776, t1888, t2717);
    (t86806, t86810, t86815, t86816, t86821, t86825, t86830, t86843, t86847, t86852, t86857, t86862)
}
