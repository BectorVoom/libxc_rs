//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 932/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk932<F: Float>(t1539: F, t5878: F, t3071: F, t10930: F, t20234: F, t974: F, t20217: F, t998: F, t10942: F, t21510: F, t4583: F, t4582: F, t1041: F, t10413: F, t14117: F, t14160: F, t14203: F, t1618: F, t17885: F, t17907: F, t18005: F, t18008: F, t18030: F, t973: F) -> (F, F, F, F, F, F) {
    let t21531 = t5878 * t1539;
    let t21532 = t3071 * t21531;
    let t21537 = t10930 * t20234;
    let t21538 = t974 * t21537;
    let t21541 = t998 * t20217;
    let t21542 = t974 * t21541;
    let t21545 = t10942 * t20234;
    let t21546 = t974 * t21545;
    let t21550 = t4583 * t21510;
    let t21551 = t4582 * t21550;
    let t21560 = -t10413 * t21532 / 1536.0 + 5.0 / 6912.0 * t17885 - t14117 / 4608.0 - t973 * t21538 / 36.0 + t973 * t21542 / 288.0 + 7.0 / 648.0 * t973 * t21546 - t17907 / 1152.0 - t1041 * t21551 / 768.0 + t18030 * t1618 / 1024.0 - t14160 / 432.0 + t18005 / 1536.0 + t18008 / 1152.0 - t14203 / 6912.0;
    (t21532, t21538, t21542, t21546, t21551, t21560)
}
