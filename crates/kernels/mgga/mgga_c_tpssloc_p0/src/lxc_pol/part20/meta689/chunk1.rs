//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2612/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2612<F: Float>(t11713: F, t3503: F, t53081: F, t1210: F, t11719: F, t13969: F, t15626: F, t11529: F, t1174: F, t4729: F, t11647: F, t1731: F) -> (F, F, F, F, F) {
    let t53083 = t11713 * t3503 * t53081;
    let t53087 = t11713 * t1210 * t53081;
    let t53093 = t11719 * t13969 * t15626;
    let t53096 = t1174 * t11529 * t4729;
    let t53097 = t53096 / F::cast_from(216.0_f64);
    let t53099 = t1731 * t11647;
    (t53083, t53087, t53093, t53097, t53099)
}
