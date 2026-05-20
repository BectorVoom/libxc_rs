//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1798/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1798<F: Float>(t25261: F, t829: F, t1510: F, t22992: F, t13380: F, t232: F, t6646: F, t1888: F, t1499: F, t23002: F, t23014: F, t23026: F, t23028: F, t23032: F, t23166: F, t23169: F, t23174: F, t25239: F, t25243: F, t25246: F, t25252: F, t25256: F, t25259: F, t2617: F, t4291: F, t6660: F, t7533: F, t812: F) -> (F, F, F, F, F) {
    let t25262 = t25261 * t829;
    let t25269 = t22992 * t1510;
    let t25272 = t13380 * t232;
    let t25273 = t6646 * t25272;
    let t25274 = t1888 * t25273;
    let t25276 = -F::cast_from(0.82246703342411321825e-2_f64) * t25239 - F::cast_from(0.82246703342411321825e-2_f64) * t25243 + F::cast_from(0.41123351671205660912e-2_f64) * t25246 + F::cast_from(0.49348022005446793095e-1_f64) * t25252 + F::cast_from(0.19190897446562641759e-1_f64) * t23002 - t812 * t25256 - F::cast_from(0.41123351671205660912e-2_f64) * t25259 - t4291 * t25262 + t23014 - F::cast_from(0.41123351671205660912e-2_f64) * t23026 - F::cast_from(0.19190897446562641759e-1_f64) * t23028 + t23032 + F::cast_from(0.82246703342411321824e-2_f64) * t23166 + F::cast_from(0.38381794893125283518e-1_f64) * t23169 - t2617 * t7533 - t812 * t25269 - t23174 + t1499 * t6660 - F::cast_from(0.82246703342411321825e-2_f64) * t25274;
    (t25262, t25269, t25272, t25273, t25276)
}
