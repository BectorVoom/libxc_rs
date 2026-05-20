//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2611/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2611<F: Float>(t11678: F, t11697: F, t15559: F, t15713: F, t3577: F, t45124: F, t1213: F, t1735: F, t248: F, t45017: F, t10477: F, t1742: F) -> (F, F, F, F) {
    let t53064 = t11678 * t11697 * t15559;
    let t53067 = t3577 * t45124 * t15713;
    let t53079 = t1213 * t248 * t45017 * t1735;
    let t53081 = t1742 * t10477;
    (t53064, t53067, t53079, t53081)
}
