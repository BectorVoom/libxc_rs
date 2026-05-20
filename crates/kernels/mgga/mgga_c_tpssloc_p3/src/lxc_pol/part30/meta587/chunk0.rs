//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1966/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1966<F: Float>(t17095: F, t225: F, t17098: F, t18065: F, t17579: F, t18048: F, t1597: F, t976: F, t18057: F, t18059: F, t18053: F, t112: F, t20148: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t59519 = t17095 * t225;
    let t59537 = t17098 * t225;
    let t60971 = t18065 * t225;
    let t61058 = t17579 * t225;
    let t61061 = t18048 * t225;
    let t61066 = t976 * t1597;
    let t61621 = t18057 * t225;
    let t61646 = t18059 * t225;
    let t63215 = t18053 * t225;
    let t66958 = t20148 * t112;
    (t59519, t59537, t60971, t61058, t61061, t61066, t61621, t61646, t63215, t66958)
}
