//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2607/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2607<F: Float>(t1227: F, t13969: F, t15454: F, t4973: F, t49850: F, t11678: F, t11697: F, t15559: F, t15713: F, t3577: F, t45124: F, t1213: F, t1735: F, t248: F, t45017: F) -> (F, F, F, F, F) {
    let t53026 = t1227 * t13969 * t15454;
    let t53033 = t1227 * t49850 * t4973;
    let t53064 = t11678 * t11697 * t15559;
    let t53067 = t3577 * t45124 * t15713;
    let t53079 = t1213 * t248 * t45017 * t1735;
    (t53026, t53033, t53064, t53067, t53079)
}
