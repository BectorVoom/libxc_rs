//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1018/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1018<F: Float>(t11018: F, t2515: F, t141: F, t11022: F, t11008: F, t8633: F, t11031: F, t861: F, t11035: F, t11004: F, t11040: F, t854: F, t1415: F, t8684: F, t2488: F, t8678: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11055 = t2515 * t11018;
    let t11056 = t141 * t11055;
    let t11058 = t2515 * t11022;
    let t11059 = t141 * t11058;
    let t11061 = t8633 * t11008;
    let t11062 = t141 * t11061;
    let t11064 = t861 * t11031;
    let t11065 = t141 * t11064;
    let t11067 = t861 * t11035;
    let t11068 = t141 * t11067;
    let t11071 = 0.39862222222222222222e0 * t11004;
    let t11080 = t854 * t11040;
    let t11082 = t8684 * t1415;
    let t11083 = t11082 * t2488;
    let t11085 = t8678 * t1415;
    (t11056, t11059, t11062, t11065, t11068, t11071, t11080, t11083, t11085)
}
