//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2622/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2622<F: Float>(t15338: F, t18427: F, t3447: F, t22032: F, t3448: F, t11570: F, t20234: F, t1409: F, t15293: F, t18416: F, t18420: F, t18469: F, t18542: F, t3449: F, t3450: F, t4900: F, t4908: F, t4919: F, t4928: F, t52140: F, t71168: F, t71172: F, t71181: F, t71185: F, t73138: F) -> (F, F, F) {
    let t73199 = t3447 * t15338 * t18427;
    let t73201 = t3448 * t22032;
    let t73225 = t11570 * t20234;
    let t73252 = F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t18416 * t18542 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t18416 * t15293 - F::cast_from(0.49999999999999999998e-2_f64) * t3447 * t4908 * t71168 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t3449 * t73225 - F::cast_from(0.66666666666666666664e-2_f64) * t3447 * t4908 * t71172 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4900 * t71181 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4900 * t71185 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t52140 * t18469 + F::cast_from(0.66666666666666666665e-2_f64) * t3447 * t4900 * t73138 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4919 * t3450 * t1409 * t4928 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t18420 * t18542;
    (t73199, t73201, t73252)
}
