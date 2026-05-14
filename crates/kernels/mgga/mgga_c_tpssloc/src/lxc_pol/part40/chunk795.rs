//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 795/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk795<F: Float>(t25: F, t1268: F, t1458: F, t4028: F, t5450: F, t5456: F, t5493: F, t88: F, t5155: F, t5158: F, t1799: F, t5122: F, t5169: F, t1408: F, t3664: F, t514: F, t5397: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t6295 = 2.0 * t1268 * t5493 + 4.0 * t1458 * t4028 + 2.0 * t5456 * t88 + t5450;
    let t6299 = 0.11696447245269292414e1 * t5155;
    let t6300 = 0.36622894612013090108e-3 * t5158;
    let t6301 = t5122 * t1799;
    let t6304 = 2.0 * t5169;
    let t6305 = t1408 * t1408;
    let t6311 = piecewise3(t26, 0.0, 4.0 / 9.0 * t3664 * t6305 + 4.0 / 3.0 * t514 * t5397);
    (t6295, t6299, t6300, t6301, t6304, t6305, t6311)
}
