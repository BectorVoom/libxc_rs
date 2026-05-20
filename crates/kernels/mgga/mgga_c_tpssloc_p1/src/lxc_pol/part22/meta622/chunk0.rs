//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2155/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2155<F: Float>(t1174: F, t5045: F, t698: F, t3540: F, t4966: F, t11647: F, t1744: F, t3247: F, t475: F, t15032: F, t3576: F, t11713: F, t11716: F, t53081: F) -> (F, F, F, F, F, F) {
    let t53270 = t1174 * t698 * t5045;
    let t53271 = t53270 / F::new(432.0);
    let t53272 = t4966 * t3540;
    let t53273 = t53272 / F::new(4608.0);
    let t53274 = t1744 * t11647;
    let t53298 = t475 * t3247;
    let t53322 = t15032 * t3576;
    let t53336 = t11713 * t11716 * t53081;
    (t53271, t53273, t53274, t53298, t53322, t53336)
}
