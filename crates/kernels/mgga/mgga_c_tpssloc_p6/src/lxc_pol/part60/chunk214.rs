//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 214/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk214<F: Float>(t419: F, t409: F, t410: F, t1086: F, t407: F, t281: F, t415: F, t904: F, t241: F, t457: F, t422: F, t432: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1097 = t419 * t419;
    let t1098 = F::cast_from(1.0_f64) / t1097;
    let t1099 = t409 * t1098;
    let t1100 = F::cast_from(1.0_f64) / t410;
    let t1105 = F::cast_from(0.29896666666666666667e0_f64) * t1086;
    let t1107 = F::sqrt(t407);
    let t1111 = t281 * t904 * t415;
    let t1112 = F::cast_from(0.82156666666666666667e-1_f64) * t1111;
    let t1113 = t241 * t457;
    let t1118 = F::cast_from(1.0_f64) / t422;
    let t1122 = F::cast_from(0.17123333333333333333e-1_f64) * t1086;
    let t1127 = t432 * t432;
    (t1097, t1098, t1099, t1100, t1105, t1107, t1111, t1112, t1113, t1118, t1122, t1127)
}
