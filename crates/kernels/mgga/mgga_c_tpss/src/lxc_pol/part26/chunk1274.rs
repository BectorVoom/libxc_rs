//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1274/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1274<F: Float>(t12535: F, t6002: F, t1100: F, t139: F, t20808: F, t4052: F, t20802: F, t3028: F, t1141: F, t1569: F, t2738: F, t19128: F, t6513: F, t3048: F, t6509: F, t20862: F, t6030: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t68476 = t6002 * t12535 / 432.0;
    let t68489 = t20808 * t139 * t1100 * t4052 / 216.0;
    let t68511 = t20802 * t3028 / 162.0;
    let t68522 = t1141 * sigma2 * t1569 * t2738;
    let t68532 = t6513 * t19128;
    let t68557 = t3048 * t6509;
    let t68572 = t20862 * t6030;
    (t68476, t68489, t68511, t68522, t68532, t68557, t68572)
}
