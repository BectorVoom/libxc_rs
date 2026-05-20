//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1153/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1153<F: Float>(t3788: F, t6388: F, t6936: F, t1339: F, t6420: F, t6417: F, t6945: F, t1827: F, t26233: F, t6415: F, t22839: F, t6371: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28057 = t3788 * t6388;
    let t28058 = t6936 * t28057;
    let t28060 = t1339 * t6420;
    let t28061 = t6936 * t28060;
    let t28063 = t6945 * t6417;
    let t28065 = t26233 * t1827;
    let t28067 = t1339 * t6415;
    let t28068 = t6936 * t28067;
    let t28070 = t22839 * t6371;
    (t28057, t28058, t28060, t28061, t28063, t28065, t28067, t28068, t28070)
}
