//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 977/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk977<F: Float>(t4646: F, t600: F, t4645: F, t7594: F, t640: F, t3508: F, t3532: F, t4670: F, t2073: F, t4669: F, t4649: F, t7613: F) -> (F, F, F, F, F, F) {
    let t13483 = t600 * t4646;
    let t13485 = t7594 * t4645;
    let t13486 = t13485 * t640;
    let t13489 = t3508 * t3532;
    let t13492 = t600 * t4670;
    let t13494 = t2073 * t4669;
    let t13495 = t13494 * t640;
    let t13500 = t7613 * t4649;
    (t13483, t13486, t13489, t13492, t13495, t13500)
}
