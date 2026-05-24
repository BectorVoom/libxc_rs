//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 512/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk512<F: Float>(t2073: F, t2074: F, t588: F, t99: F, t633: F, t100: F, t1990: F, t107: F, t636: F, t108: F, t101: F, t105: F, t631: F, t634: F, t97: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2075 = t2073 * t2074;
    let t2078 = tau0 * t588;
    let t2083 = F::new(1.0) / t99;
    let t2084 = t633 * t633;
    let t2085 = t2083 * t2084;
    let t2088 = t100 * t1990;
    let t2091 = F::new(1.0) / t107;
    let t2092 = t636 * t636;
    let t2093 = t2091 * t2092;
    let t2096 = -t1990;
    let t2097 = t108 * t2096;
    let t2100 = F::new(40.0) / F::new(9.0) * t2078 * t101 - F::new(50.0) / F::new(9.0) * t631 * t634 + F::new(10.0) / F::new(9.0) * t97 * t2085 + F::new(5.0) / F::new(3.0) * t97 * t2088 + F::new(10.0) / F::new(9.0) * t105 * t2093 + F::new(5.0) / F::new(3.0) * t105 * t2097;
    (t2075, t2078, t2083, t2084, t2091, t2092, t2093, t2096, t2097, t2100)
}
