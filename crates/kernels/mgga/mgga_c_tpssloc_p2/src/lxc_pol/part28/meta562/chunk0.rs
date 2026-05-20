//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1834/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1834<F: Float>(t1408: F, t2745: F, t25365: F, t81547: F, t1530: F, t2553: F, t22960: F, t12971: F, t25: F, t2379: F, t4255: F, t606: F, t870: F) -> (F, F, F, F, F, F, F) {
    let t86806 = t1408 * t2745;
    let t86810 = t81547 * t25365;
    let t86815 = t1530 * t2553;
    let t86816 = t22960 * t86815;
    let t86821 = t25 * t12971;
    let t86825 = t1408 * t2379;
    let t86830 = t870 * t606 * t4255;
    (t86806, t86810, t86815, t86816, t86821, t86825, t86830)
}
