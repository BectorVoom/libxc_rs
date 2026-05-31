//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 979/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk979<F: Float>(t112611: F, t1983: F, t2095: F, t22578: F, t8640: F, t31297: F, t6876: F, t31670: F, t31650: F, t6883: F, t31608: F, t1377: F, t7213: F) -> (F, F, F, F, F, F, F) {
    let t115275 = t1983 * t2095 * t112611;
    let t115277 = t1983 * t8640 * t22578;
    let t115279 = F::cast_from(2.0_f64) * t6876 * t31297;
    let t115283 = F::cast_from(2.0_f64) * t6876 * t31670;
    let t115292 = t6883 * t31650;
    let t115294 = t6883 * t31608;
    let t115296 = t1377 * t7213;
    (t115275, t115277, t115279, t115283, t115292, t115294, t115296)
}
