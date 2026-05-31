//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 433/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk433<F: Float>(t1535: F, t434: F, t1075: F, t1078: F, t1503: F, t1510: F, t1513: F, t1516: F) -> (F, F) {
    let t1536 = t1535 * t434;
    let t1542 = F::cast_from(0.258925e1_f64) * t1510 - t1075 + F::cast_from(0.301925e0_f64) * t1503 + F::cast_from(0.16504875e0_f64) * t1513 - t1078 + F::cast_from(0.82785e-1_f64) * t1516;
    (t1536, t1542)
}
