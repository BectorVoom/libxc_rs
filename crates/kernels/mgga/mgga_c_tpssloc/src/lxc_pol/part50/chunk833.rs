//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 833/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk833<F: Float>(t23478: F, t364: F, t23477: F, t6721: F, t6739: F, t6741: F, t344: F, t6729: F, t6740: F, t3103: F, t6755: F, t3034: F, t371: F, t1930: F, t1940: F, t3046: F) -> (F, F, F, F, F, F, F) {
    let t23479 = t23478 * t364;
    let t23480 = t23477 * t23479;
    let t23482 = t6721 * t6739;
    let t23483 = t23482 * t6741;
    let t23488 = t6729 * t344;
    let t23489 = t6740 * t23488;
    let t23500 = t6755 * t3103;
    let t23508 = 1.0 / t3034 / t371;
    let t23509 = t1930 * t23508;
    let t23528 = t1940 * t3046;
    (t23479, t23480, t23483, t23489, t23500, t23509, t23528)
}
