//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3059/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3059<F: Float>(t18893: F, t3359: F, t11303: F, t11350: F, t1136: F, t11415: F, t11420: F, t15117: F, t15146: F, t15159: F, t15165: F, t15168: F, t15172: F, t1683: F, t18631: F, t18634: F, t18637: F, t18640: F, t18643: F, t18644: F, t18650: F, t18894: F, t3332: F, t3333: F, t3351: F, t3357: F, t44214: F, t44361: F, t4824: F, t51427: F, t51599: F, t51604: F, t6037: F, t6053: F, t6056: F) -> F {
    let t63502 = t18893 * t3359;
    let t63506 = -F::cast_from(0.38596750796862084161e3_f64) * t51427 * t15159 + F::cast_from(0.12865583598954028054e3_f64) * t51599 * t4824 + F::cast_from(0.12865583598954028054e3_f64) * t15146 * t15165 + F::cast_from(0.64327917994770140268e2_f64) * t15146 * t15168 + F::cast_from(0.4138081033541872024e4_f64) * t51604 * t15172 + F::new(12.0) * t11415 * t18631 + F::new(6.0) * t3357 * t6037 * t3351 + F::cast_from(0.11579025239058625248e4_f64) * t11350 * t6056 * t3333 - F::new(8.0) * t11303 * t18634 - F::new(4.0) * t3332 * t1683 * t15117 - F::cast_from(0.38596750796862084162e3_f64) * t44214 * t18637 - F::cast_from(0.19298375398431042081e3_f64) * t11420 * t6056 * t3351 - F::cast_from(0.24828486201251232145e5_f64) * t44361 * t18650 * t3333 - F::new(4.0) * t11303 * t18640 - F::new(4.0) * t3332 * t18894 * t1136 - F::new(2.0) * t3332 * t6053 * t3351 - F::cast_from(0.19298375398431042081e3_f64) * t11420 * t18643 * t3333 + F::cast_from(0.64327917994770140268e2_f64) * t11415 * t18644 + F::cast_from(0.64327917994770140268e2_f64) * t3357 * t63502 * t1136;
    t63506
}
