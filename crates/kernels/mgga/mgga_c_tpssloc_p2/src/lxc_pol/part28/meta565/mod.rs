//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1840;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta565<F: Float>(t1888: F, t25045: F, t82159: F, t6562: F, t7488: F, t82133: F, t25225: F, t6547: F, t23168: F, t25338: F, t23012: F, t7485: F, t23270: F, t2719: F, t46488: F, t25046: F, t6579: F, t1484: F, t2717: F, t22986: F, t7489: F, t13460: F, t1880: F, t6553: F, t6571: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t86933, t86940, t86942, t86950, t86955) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1840::<F>(t1888, t25045, t82159, t6562, t7488, t82133, t25225, t6547, t23168, t25338, t23012, t7485);
        let (t86961, t86967, t86972, t86991, t86997) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1841::<F>(t1888, t23270, t2719, t46488, t25046, t6579, t1484, t2717, t22986, t23012, t7489, t13460, t1880, t6553, t6571);
    (t86933, t86940, t86942, t86950, t86955, t86961, t86967, t86972, t86991, t86997)
}
