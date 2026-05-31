//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1439/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1439<F: Float>(t423: F, t44342: F, t44355: F, t3330: F, t3355: F, t427: F, t1129: F, t11310: F, t11311: F, t11345: F, t11350: F, t11352: F, t11366: F, t1137: F, t1138: F, t11410: F, t11421: F, t1156: F, t3327: F, t3334: F, t3352: F, t3359: F, t3360: F, t3376: F, t3378: F, t3403: F, t436: F, t43679: F, t43692: F, t43951: F, t44142: F, t44168: F, t44202: F, t44205: F, t44211: F, t44214: F, t44220: F, t44223: F, t44243: F, t44258: F, t44274: F, t44289: F, t44295: F, t44300: F, t44314: F, t44327: F) -> (F, F) {
    let t44358 = F::cast_from(0.621814e-1_f64) * (t44342 + t44355) * t423;
    let t44361 = t427 / t3355 / t3330;
    let t44366 = -F::cast_from(0.35089341735807877242e1_f64) * t3376 * t44168 * t1156 - F::cast_from(0.70178683471615754484e1_f64) * t44202 * t3378 - F::cast_from(0.4155806185363551302e3_f64) * t44205 * t11366 + F::cast_from(0.6233709278045326953e3_f64) * t11310 * t43679 * t3403 - F::cast_from(12.0_f64) * t44211 * t3334 - F::cast_from(0.77193501593724168322e3_f64) * t44214 * t11421 + F::cast_from(0.11579025239058625248e4_f64) * t11350 * t44142 * t3359 + F::cast_from(0.4101607543286562663e4_f64) * t44220 * t11311 + F::cast_from(0.91082604192152556044e5_f64) * t44223 * t43679 * t43692 + F::cast_from(4.0_f64) * t3327 * t11345 + F::cast_from(1.0_f64) * t1129 * (t44243 + t44258 + t44274 + t44289) * t1137 + F::cast_from(4.0_f64) * t44295 * t1138 + F::cast_from(6.0_f64) * t11410 * t3352 + F::cast_from(0.1929837539843104208e3_f64) * t44300 * t3360 - F::cast_from(0.310907e-1_f64) * (t44314 + t44327) * t436 + t44358 - F::cast_from(0.24828486201251232145e5_f64) * t44361 * t44142 * t11352 - F::cast_from(0.19751673498613801407e-1_f64) * t43951;
    (t44358, t44366)
}
