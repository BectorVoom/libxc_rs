//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2605/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2605<F: Float>(t11665: F, t15572: F, t3515: F, t4983: F, t49850: F, t11818: F, t1213: F, t248: F, t5012: F, t11801: F, t5024: F, t11820: F, t5019: F) -> (F, F, F, F, F) {
    let t52942 = t11665 * t15572;
    let t52952 = t3515 * t49850 * t4983;
    let t52973 = t1213 * t248 * t11818 * t5012;
    let t52975 = t5024 * t11801;
    let t52987 = t5019 * t11820;
    (t52942, t52952, t52973, t52975, t52987)
}
