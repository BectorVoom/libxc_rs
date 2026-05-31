//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1127/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1127<F: Float>(t13330: F, t4283: F, t3931: F, t242: F, t5064: F, t9523: F, t1125: F, t1130: F, t15485: F, t15489: F, t15493: F, t15500: F, t15504: F, t15506: F, t4248: F, t4258: F, t4265: F, t4280: F, t9535: F) -> F {
    let t15510 = t4283 * t13330;
    let t15511 = t3931 * t15510;
    let t15515 = t242 * t9523 * t5064;
    let t15516 = t1125 * t15515;
    let t15518 = -t15485 / F::cast_from(432.0_f64) + t15489 / F::cast_from(2304.0_f64) + t9535 - F::cast_from(19.0_f64) / F::cast_from(2592.0_f64) * t15493 * t1130 - t4258 * t4248 / F::cast_from(288.0_f64) - t15500 / F::cast_from(3456.0_f64) - t15504 / F::cast_from(6912.0_f64) + F::cast_from(19.0_f64) / F::cast_from(2592.0_f64) * t15506 - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t4265 * t4280 - t1125 * t15511 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t15516;
    t15518
}
