//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 863/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk863<F: Float>(t22479: F, t89: F, t2040: F, t31540: F, t7050: F, t2314: F, t31747: F, t531: F, t8639: F, t1983: F, t22596: F, t22581: F, t8607: F, t112611: F, t2095: F, t22578: F, t8640: F) -> (F, F, F, F, F, F, F) {
    let t115252 = t89 * t22479;
    let t115254 = 2.0 * t115252 * t2040;
    let t115256 = 4.0 * t31540 * t7050;
    let t115261 = 4.0 * t2314 * t31747;
    let t115262 = t531 * t8639;
    let t115265 = 6.0 * t1983 * t115262 * t22596;
    let t115271 = 2.0 * t8607 * t22581;
    let t115275 = t1983 * t2095 * t112611;
    let t115277 = t1983 * t8640 * t22578;
    (t115254, t115256, t115261, t115265, t115271, t115275, t115277)
}
