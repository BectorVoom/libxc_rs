//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1177/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1177<F: Float>(t112943: F, t6562: F, t7488: F, t1484: F, t865: F, t22986: F, t23270: F, t30633: F, t112867: F, t1880: F, t23237: F, t32866: F, t1888: F, t4300: F, t25216: F, t30663: F) -> (F, F, F, F, F, F, F) {
    let t118830 = t6562 * t112943 * t7488;
    let t118831 = 0.82246703342411321825e-2 * t118830;
    let t118833 = t1484 * t865;
    let t118837 = 0.6579736267392905746e-1 * t22986 * t23270 * t30633 * t118833;
    let t118838 = 0.16449340668482264365e-1 * t112867;
    let t118841 = 0.16449340668482264365e-1 * t1880 * t23237 * t32866;
    let t118847 = 0.3289868133696452873e-1 * t1888 * t23270 * t30633 * t4300;
    let t118850 = 0.16449340668482264365e-1 * t1880 * t30663 * t25216;
    (t118831, t118833, t118837, t118838, t118841, t118847, t118850)
}
