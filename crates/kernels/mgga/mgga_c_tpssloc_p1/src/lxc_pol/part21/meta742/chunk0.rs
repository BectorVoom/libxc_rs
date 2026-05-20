//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2608/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2608<F: Float>(t10477: F, t1742: F, t11713: F, t3503: F, t1210: F, t11719: F, t13969: F, t15626: F, t11529: F, t1174: F, t4729: F, t11647: F, t1731: F) -> (F, F, F, F, F, F) {
    let t53081 = t1742 * t10477;
    let t53083 = t11713 * t3503 * t53081;
    let t53087 = t11713 * t1210 * t53081;
    let t53093 = t11719 * t13969 * t15626;
    let t53096 = t1174 * t11529 * t4729;
    let t53099 = t1731 * t11647;
    (t53081, t53083, t53087, t53093, t53096, t53099)
}
