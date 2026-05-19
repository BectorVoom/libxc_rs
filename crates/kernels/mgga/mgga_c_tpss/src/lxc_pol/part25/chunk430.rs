//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 430/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk430<F: Float>(t1024: F, t1519: F, t1047: F, t1503: F, t1056: F, t1059: F, t1510: F, t1513: F, t1516: F, t1062: F) -> (F, F, F, F) {
    let t1521 = F::new(1.0) * t1024 * t1519;
    let t1523 = -t1047 + F::cast_from(0.17123333333333333333e-1_f64) * t1503;
    let t1530 = F::new(0.3529725e1) * t1510 - t1056 + F::new(0.516475e0) * t1503 + F::new(0.6311625e0) * t1513 - t1059 + F::new(0.104195e0) * t1516;
    let t1531 = t1530 * t1062;
    (t1521, t1523, t1530, t1531)
}
