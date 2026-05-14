//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 959/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk959<F: Float>(t30958: F, t30988: F, t649: F, t8319: F, t510: F, t1266: F, t8320: F, t8301: F, t9231: F, t8303: F) -> (F, F, F, F, F, F) {
    let t30989 = t30958 + t30988;
    let t30991 = t649 * t8319;
    let t30993 = 2.0 * t30991 * t510;
    let t30995 = 2.0 * t8320 * t1266;
    let t31000 = t9231 * t8301;
    let t31003 = t8301 * t8303;
    (t30989, t30991, t30993, t30995, t31000, t31003)
}
