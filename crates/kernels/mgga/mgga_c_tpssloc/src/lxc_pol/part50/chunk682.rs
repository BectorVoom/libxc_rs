//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 682/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk682<F: Float>(t1339: F, t1352: F, t6936: F, t1332: F, t2002: F, t559: F, t1338: F, t59: F) -> (F, F, F, F, F) {
    let t6937 = t1339 * t1352;
    let t6938 = t6936 * t6937;
    let t6940 = t1332 * t2002;
    let t6941 = t6940 * t559;
    let t6943 = t1338 * t59;
    (t6937, t6938, t6940, t6941, t6943)
}
