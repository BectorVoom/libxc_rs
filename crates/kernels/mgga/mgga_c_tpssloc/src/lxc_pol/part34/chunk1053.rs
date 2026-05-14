//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1053/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1053<F: Float>(t20450: F, t22833: F, t6390: F, t91388: F, t1339: F, t1824: F, t22827: F, t550: F, t6347: F, t1799: F, t6414: F, t26288: F, t6330: F, t6420: F, t1825: F, t6936: F) -> (F, F, F, F, F, F, F) {
    let t107147 = t22833 * t20450;
    let t107151 = t91388 * t6390;
    let t107159 = t22827 * t1339 * t6347 * t1824 * t550;
    let t107164 = t22827 * t1339 * t1799 * t6414 * t550;
    let t107169 = t26288 * t1339 * t6330 * t1824 * t550;
    let t107174 = t22827 * t1339 * t6420 * t1799;
    let t107178 = t6936 * t1339 * t1825 * t6414;
    (t107147, t107151, t107159, t107164, t107169, t107174, t107178)
}
