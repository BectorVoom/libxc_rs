//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1094/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1094<F: Float>(t242: F, t3090: F, t5068: F, t1125: F, t5072: F, t1120: F, t5231: F, t13330: F, t4283: F, t3931: F, t5064: F, t9523: F, t1130: F, t15485: F, t15489: F, t15493: F, t4248: F, t4258: F, t4265: F, t4280: F, t9535: F) -> (F,) {
    let t15499 = t242 * t3090 * t5068;
    let t15500 = t1125 * t15499;
    let t15503 = t242 * t3090 * t5072;
    let t15504 = t1125 * t15503;
    let t15506 = t5231 * t1120;
    let t15510 = t4283 * t13330;
    let t15511 = t3931 * t15510;
    let t15515 = t242 * t9523 * t5064;
    let t15516 = t1125 * t15515;
    let t15518 = -t15485 / 432.0 + t15489 / 2304.0 + t9535 - 19.0 / 2592.0 * t15493 * t1130 - t4258 * t4248 / 288.0 - t15500 / 3456.0 - t15504 / 6912.0 + 19.0 / 2592.0 * t15506 - 5.0 / 1296.0 * t4265 * t4280 - t1125 * t15511 / 2304.0 + 5.0 / 20736.0 * t15516;
    (t15518,)
}
