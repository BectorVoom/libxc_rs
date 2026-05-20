//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1800/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1800<F: Float>(t23092: F, t23149: F, t235: F, t234: F, t852: F, t776: F, t6637: F, t6552: F, t2553: F, t6638: F, t117: F, t229: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23150 = t23092 + t23149;
    let t23151 = t235 * t23150;
    let t23153 = t234 * t852;
    let t23154 = t23153 * t776;
    let t23155 = t6637 * t23154;
    let t23156 = t6552 * t23155;
    let t23158 = t6638 * t2553;
    let t23159 = t6637 * t23158;
    let t23160 = t6552 * t23159;
    let t23163 = t229 * t67 * t117;
    (t23150, t23151, t23153, t23154, t23155, t23156, t23158, t23159, t23160, t23163)
}
