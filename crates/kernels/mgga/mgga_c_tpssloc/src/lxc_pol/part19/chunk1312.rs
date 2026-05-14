//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1312/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1312<F: Float>(t300: F, t44115: F, t44138: F, t44198: F, t44366: F, t1164: F, t3396: F, t3422: F, t43994: F, t43997: F, t44000: F, t44002: F, t44006: F, t44072: F, t44080: F, t44082: F, t44085: F, t44089: F, t44092: F) -> (F, F, F) {
    let t44369 = t300 * (t44115 + t44138 + t44198 + t44366);
    let t44372 = 0.21053605041484726346e2 * t1164 * t3422 * t3396;
    let t44373 = t43994 - t43997 - t44000 + t44002 + t44006 + t44072 + t44080 + t44082 - t44085 - t44089 + t44092 + t44369 - t44372;
    (t44369, t44372, t44373)
}
