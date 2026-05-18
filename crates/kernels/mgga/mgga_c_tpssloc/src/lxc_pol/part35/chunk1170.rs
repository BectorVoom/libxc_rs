//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1170/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1170<F: Float>(t131: F, t27505: F, t467: F, t225: F, t8034: F, t7327: F, t221: F, t4899: F, t2127: F, t2135: F, t477: F, t3242: F, t491: F) -> (F, F, F, F, F, F, F) {
    let t27506 = t27505 * t131;
    let t27507 = t27506 * t467;
    let t27516 = t8034 * t225;
    let t27536 = t8034 * t7327;
    let t27548 = t221 * t4899;
    let t27549 = t2127 * t27548;
    let t27550 = t2135 * t477;
    let t27551 = t491 * t3242;
    (t27506, t27507, t27516, t27536, t27549, t27550, t27551)
}
