//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1190/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1190<F: Float>(t24745: F, t7363: F, t3502: F, t491: F, t24813: F, t1209: F, t1090: F, t7376: F, t221: F, t4899: F, t2127: F, t2135: F, t477: F) -> (F, F, F, F, F, F, F) {
    let t27454 = t24745 * t7363;
    let t27488 = t3502 * t491;
    let t27489 = t24813 * t27488;
    let t27495 = t1209 * t491;
    let t27496 = t24813 * t27495;
    let t27532 = t7376 * t1090;
    let t27548 = t221 * t4899;
    let t27549 = t2127 * t27548;
    let t27550 = t2135 * t477;
    (t27454, t27489, t27495, t27496, t27532, t27549, t27550)
}
