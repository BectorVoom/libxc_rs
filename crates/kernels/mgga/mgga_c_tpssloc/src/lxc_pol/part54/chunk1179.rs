//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1179/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1179<F: Float>(t32809: F, t6547: F, t1880: F, t8335: F, t87782: F, t6562: F, t86893: F, t214: F, t7510: F, t6572: F, t32867: F, t112945: F, t112948: F, t6552: F, t6555: F, t32875: F) -> (F, F, F, F, F, F, F, F, F) {
    let t118893 = t6547 * t32809;
    let t118894 = 0.38381794893125283518e-1 * t118893;
    let t118901 = 0.16449340668482264365e-1 * t1880 * t87782 * t8335;
    let t118903 = t6562 * t86893 * t8335;
    let t118904 = 0.82246703342411321825e-2 * t118903;
    let t118910 = t214 * t7510;
    let t118913 = 0.16449340668482264365e-1 * t1880 * t118910 * t6572;
    let t118915 = t6547 * t32867;
    let t118916 = 0.38381794893125283518e-1 * t118915;
    let t118917 = 0.16449340668482264365e-1 * t112945;
    let t118918 = 0.82246703342411321825e-2 * t112948;
    let t118924 = 0.3289868133696452873e-1 * t6552 * t118910 * t6555;
    let t118927 = t6547 * t32875;
    (t118894, t118901, t118904, t118913, t118916, t118917, t118918, t118924, t118927)
}
