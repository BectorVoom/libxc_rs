//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1071/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1071<F: Float>(t32867: F, t6547: F, t112945: F, t112948: F, t118910: F, t6552: F, t6555: F, t32875: F, t32808: F, t6562: F, t794: F, t25341: F, t30663: F, t112943: F, t23164: F, t7479: F) -> (F, F, F, F, F, F, F, F) {
    let t118915 = t6547 * t32867;
    let t118916 = 0.38381794893125283518e-1 * t118915;
    let t118917 = 0.16449340668482264365e-1 * t112945;
    let t118918 = 0.82246703342411321825e-2 * t112948;
    let t118924 = 0.3289868133696452873e-1 * t6552 * t118910 * t6555;
    let t118927 = t6547 * t32875;
    let t118928 = 0.38381794893125283518e-1 * t118927;
    let t118934 = t6562 * t794 * t32808;
    let t118935 = 0.82246703342411321825e-2 * t118934;
    let t118938 = 0.3289868133696452873e-1 * t6552 * t30663 * t25341;
    let t118940 = t23164 * t112943 * t7479;
    (t118916, t118917, t118918, t118924, t118928, t118935, t118938, t118940)
}
